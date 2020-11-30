/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy.protocols

import software.amazon.smithy.codegen.core.Symbol
import software.amazon.smithy.model.Model
import software.amazon.smithy.model.knowledge.OperationIndex
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.Shape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.model.traits.TimestampFormatTrait
import software.amazon.smithy.rust.codegen.lang.RustType
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.lang.rust
import software.amazon.smithy.rust.codegen.lang.rustBlock
import software.amazon.smithy.rust.codegen.lang.stripOuter
import software.amazon.smithy.rust.codegen.lang.withBlock
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.RustSymbolProvider
import software.amazon.smithy.rust.codegen.smithy.Serializers
import software.amazon.smithy.rust.codegen.smithy.WrappingSymbolProvider
import software.amazon.smithy.rust.codegen.smithy.generators.HttpProtocolGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolConfig
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolGeneratorFactory
import software.amazon.smithy.rust.codegen.smithy.generators.ProtocolSupport
import software.amazon.smithy.rust.codegen.smithy.generators.errorSymbol
import software.amazon.smithy.rust.codegen.smithy.locatedIn
import software.amazon.smithy.rust.codegen.smithy.rustType
import software.amazon.smithy.rust.codegen.smithy.traits.InputBodyTrait
import software.amazon.smithy.rust.codegen.smithy.traits.OutputBodyTrait
import software.amazon.smithy.rust.codegen.smithy.traits.SyntheticOutputTrait
import software.amazon.smithy.rust.codegen.smithy.transformers.OperationNormalizer
import software.amazon.smithy.rust.codegen.smithy.transformers.StructureModifier
import software.amazon.smithy.rust.codegen.util.dq

sealed class AwsJsonVersion {
    abstract val value: String

    object Json10 : AwsJsonVersion() {
        override val value = "1.0"
    }

    object Json11 : AwsJsonVersion() {
        override val value = "1.1"
    }
}

class BasicAwsJsonFactory(private val version: AwsJsonVersion) : ProtocolGeneratorFactory<BasicAwsJsonGenerator> {
    override fun buildProtocolGenerator(
        protocolConfig: ProtocolConfig
    ): BasicAwsJsonGenerator = BasicAwsJsonGenerator(protocolConfig, version)

    private val shapeIfHasMembers: StructureModifier = { shape: StructureShape? ->
        if (shape?.members().isNullOrEmpty()) {
            null
        } else {
            shape
        }
    }

    override fun transformModel(model: Model): Model {
        // For AwsJson10, the body matches 1:1 with the input
        return OperationNormalizer(model).transformModel(
            inputBodyFactory = shapeIfHasMembers,
            outputBodyFactory = shapeIfHasMembers
        )
    }

    override fun symbolProvider(model: Model, base: RustSymbolProvider): RustSymbolProvider {
        return JsonSerializerSymbolProvider(
            model,
            SyntheticBodySymbolProvider(model, base),
            TimestampFormatTrait.Format.EPOCH_SECONDS
        )
    }

    override fun support(): ProtocolSupport =
        ProtocolSupport(requestBodySerialization = true, requestDeserialization = true)
}

/**
 * SyntheticBodySymbolProvider makes two modifications:
 * 1. Body shapes are moved to `serializer.rs`
 * 2. Body shapes take a reference to all of their members:
 * If the base structure was:
 * ```rust
 * struct {
 *   field: Option<u64>
 * }
 * ```
 * The body will generate:
 * ```rust
 * struct<'a> {
 *   field: &'a Option<u64>
 * }
 *
 * This enables the creation of a body from a reference to an input without cloning.
 */
class SyntheticBodySymbolProvider(private val model: Model, private val base: RustSymbolProvider) :
    WrappingSymbolProvider(base) {
    override fun toSymbol(shape: Shape): Symbol {
        val initialSymbol = base.toSymbol(shape)
        val override = when (shape) {
            is StructureShape -> if (shape.hasTrait(InputBodyTrait::class.java) || shape.hasTrait(OutputBodyTrait::class.java)) {
                initialSymbol.toBuilder().locatedIn(Serializers).build()
            } else null
            is MemberShape -> {
                val container = model.expectShape(shape.container)
                if (container.hasTrait(InputBodyTrait::class.java)) {
                    initialSymbol.toBuilder().rustType(
                        RustType.Reference(
                            lifetime = "a",
                            value = initialSymbol.rustType().stripOuter<RustType.Box>()
                        )
                    ).build()
                } else {
                    null
                }
            }
            else -> null
        }
        return override ?: initialSymbol
    }
}

class BasicAwsJsonGenerator(
    private val protocolConfig: ProtocolConfig,
    private val awsJsonVersion: AwsJsonVersion
) : HttpProtocolGenerator(protocolConfig) {
    private val model = protocolConfig.model
    private val symbolProvider = protocolConfig.symbolProvider
    private val operationIndex = OperationIndex.of(model)

    override fun toHttpRequestImpl(
        implBlockWriter: RustWriter,
        operationShape: OperationShape,
        inputShape: StructureShape
    ) {
        httpBuilderFun(implBlockWriter) {
            write("let builder = \$T::new();", RuntimeType.HttpRequestBuilder)
            write(
                """
                builder
                   .method("POST")
                   .header("Content-Type", "application/x-amz-json-${awsJsonVersion.value}")
                   .header("X-Amz-Target", "${protocolConfig.serviceShape.id.name}.${operationShape.id.name}")
               """.trimMargin()
            )
        }
    }

    override fun fromResponse(
        writer: RustWriter,
        operationShape: OperationShape
    ) {
        val outputShape = operationIndex.getOutput(operationShape).get()
        val outputSymbol = symbolProvider.toSymbol(outputShape)
        val errorSymbol = operationShape.errorSymbol(symbolProvider)
        val bodyId = outputShape.expectTrait(SyntheticOutputTrait::class.java).body
        val bodyShape = bodyId?.let { model.expectShape(bodyId, StructureShape::class.java) }
        writer.rustBlock("if \$T::is_error(&response)", RuntimeType.ErrorCode) {
            writer.write(
                "let body: \$T = \$T(response.body().as_ref()).unwrap();",
                RuntimeType.SerdeJson("Value"),
                RuntimeType.SerdeJson("from_slice")
            )
            rust(
                "let error_code = \$T::error_type_from_header(&response).map_err(|e|\$T::Unmodeled(Box::new(e)))?;",
                RuntimeType.ErrorCode,
                errorSymbol
            )
            rust("let error_code = error_code.or_else(||\$T::error_type_from_body(&body));", RuntimeType.ErrorCode)
            write("let error_code = error_code.ok_or(\$T::Unknown(\"no error code\".to_string()))?;", errorSymbol)

            rust("let error_code = \$T::sanitize_error_code(error_code);", RuntimeType.ErrorCode)
            withBlock("return Err(match error_code {", "})") {
                operationShape.errors.forEach { error ->
                    rustBlock("${error.name.dq()} => match \$T(body)", RuntimeType.SerdeJson("from_value")) {
                        val variantName = symbolProvider.toSymbol(model.expectShape(error)).name
                        write(
                            "Ok(body) => \$T::$variantName(body),",
                            errorSymbol
                        )
                        write("Err(e) => \$T::Unmodeled(Box::new(e))", errorSymbol)
                    }
                }
                write("unknown => \$T::Unknown(unknown.to_string())", errorSymbol)
            }
            // write("return Err(\$T::Unknown(\"Invalid status code (todo)\".to_string()))", errorSymbol)
        }
        bodyShape?.also {
            val symbol = symbolProvider.toSymbol(it)
            writer.write(
                "let body: \$T = \$T(response.body().as_ref()).unwrap();",
                symbol,
                RuntimeType.SerdeJson("from_slice")
            )
        }
        writer.withBlock("Ok(", ")") {
            rustBlock("\$T", outputSymbol) {
                bodyShape?.members().orEmpty().forEach { member ->
                    val name = symbolProvider.toMemberName(member)
                    write("$name: body.$name,")
                }
            }
        }
    }

    override fun toBodyImpl(implBlockWriter: RustWriter, inputShape: StructureShape, inputBody: StructureShape?) {
        if (inputBody == null) {
            bodyBuilderFun(implBlockWriter) {
                write("vec![]")
            }
            return
        }
        val bodySymbol = protocolConfig.symbolProvider.toSymbol(inputBody)
        implBlockWriter.rustBlock("fn body(&self) -> \$T", bodySymbol) {
            rustBlock("\$T", bodySymbol) {
                for (member in inputBody.members()) {
                    val name = protocolConfig.symbolProvider.toMemberName(member)
                    write("$name: &self.$name,")
                }
            }
        }
        bodyBuilderFun(implBlockWriter) {
            write("\$T(&self.body()).expect(\"serialization should succeed\")", RuntimeType.SerdeJson("to_vec"))
        }
    }
}
