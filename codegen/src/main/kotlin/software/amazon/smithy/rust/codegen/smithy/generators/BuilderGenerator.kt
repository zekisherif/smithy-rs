package software.amazon.smithy.rust.codegen.smithy.generators

import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.StructureShape
import software.amazon.smithy.rust.codegen.lang.RustType
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.lang.conditionalBlock
import software.amazon.smithy.rust.codegen.lang.docs
import software.amazon.smithy.rust.codegen.lang.documentShape
import software.amazon.smithy.rust.codegen.lang.render
import software.amazon.smithy.rust.codegen.lang.rustBlock
import software.amazon.smithy.rust.codegen.lang.stripOuter
import software.amazon.smithy.rust.codegen.lang.withBlock
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.RustSymbolProvider
import software.amazon.smithy.rust.codegen.smithy.canUseDefault
import software.amazon.smithy.rust.codegen.smithy.isOptional
import software.amazon.smithy.rust.codegen.smithy.makeOptional
import software.amazon.smithy.rust.codegen.smithy.rustType
import software.amazon.smithy.rust.codegen.util.dq
import software.amazon.smithy.rust.codegen.util.inputShape

fun StructureShape.builderSymbol(symbolProvider: RustSymbolProvider): RuntimeType {
    val symbol = symbolProvider.toSymbol(this)
    return RuntimeType("Builder", null, "${symbol.namespace}::${symbol.name.toSnakeCase()}")
}

class ModelBuilderGenerator(protocolConfig: ProtocolConfig, writer: RustWriter, private val shape: StructureShape) :
    BuilderGenerator(protocolConfig.model, protocolConfig.symbolProvider, writer, shape) {
    private val symbolProvider = protocolConfig.symbolProvider
    override fun buildFn(writer: RustWriter) {
        val fallibleBuilder = StructureGenerator.fallibleBuilder(shape, symbolProvider)
        val returnType = when (fallibleBuilder) {
            true -> "Result<#T, String>"
            false -> "#T"
        }
        val outputSymbol = symbolProvider.toSymbol(shape)
        writer.docs("Consumes the builder and constructs a #D", outputSymbol)
        writer.rustBlock("pub fn build(self) -> $returnType", outputSymbol) {
            conditionalBlock("Ok(", ")", conditional = fallibleBuilder) {
                // If a wrapper is specified, use the `::new` associated function to construct the wrapper
                coreBuilder(this)
            }
        }
    }
}

class OperationInputBuilderGenerator(protocolConfig: ProtocolConfig, writer: RustWriter, private val shape: OperationShape) : BuilderGenerator(protocolConfig.model, protocolConfig.symbolProvider, writer, shape.inputShape(protocolConfig.model)) {
    private val symbolProvider = protocolConfig.symbolProvider
    override fun buildFn(writer: RustWriter) {
        val fallibleBuilder = StructureGenerator.fallibleBuilder(shape.inputShape(model), symbolProvider)
        val returnType = when (fallibleBuilder) {
            true -> "Result<#T, String>"
            false -> "#T"
        }
        val outputSymbol = symbolProvider.toSymbol(shape)

        writer.docs("Consumes the builder and constructs a #D", outputSymbol)
        writer.rustBlock("pub fn build(self) -> $returnType", outputSymbol) {
            conditionalBlock("Ok(", ")", conditional = fallibleBuilder) {
                // If a wrapper is specified, use the `::new` associated function to construct the wrapper
                withBlock("#T::new(", ")", outputSymbol) {
                    coreBuilder(this)
                }
            }
        }
    }
}

abstract class BuilderGenerator(
    val model: Model,
    private val symbolProvider: RustSymbolProvider,
    private val writer: RustWriter,
    private val shape: StructureShape
) {
    private val members: List<MemberShape> = shape.allMembers.values.toList()
    private val structureSymbol = symbolProvider.toSymbol(shape)
    fun render() {
        val symbol = symbolProvider.toSymbol(shape)
        // TODO: figure out exactly what docs we want on a the builder module
        writer.docs("See #D", symbol)
        // check(writer.namespace == shape.builderSymbol(symbolProvider).namespace)
        val segments = shape.builderSymbol(symbolProvider).namespace.split("::")
        writer.withModule(segments.last()) {
            renderBuilder(this)
        }
    }

    fun convenienceMethod(implBlock: RustWriter) {
        val builderSymbol = shape.builderSymbol(symbolProvider)
        implBlock.rustBlock("pub fn builder() -> #T", builderSymbol) {
            write("#T::default()", builderSymbol)
        }
    }

    private fun renderBuilder(writer: RustWriter) {
        val builderName = "Builder"

        val symbol = structureSymbol
        writer.docs("A builder for #D", symbol)
        writer.write("##[non_exhaustive]")
        writer.write("##[derive(Debug, Clone, Default)]")
        writer.rustBlock("pub struct $builderName") {
            members.forEach { member ->
                val memberName = symbolProvider.toMemberName(member)
                // All fields in the builder are optional
                val memberSymbol = symbolProvider.toSymbol(member).makeOptional()
                // TODO: should the builder members be public?
                write("$memberName: #T,", memberSymbol)
            }
        }

        fun builderConverter(coreType: RustType) = when (coreType) {
            is RustType.String,
            is RustType.Box -> "inp.into()"
            else -> "inp"
        }

        writer.rustBlock("impl $builderName") {
            members.forEach { member ->
                val memberName = symbolProvider.toMemberName(member)
                // All fields in the builder are optional
                val memberSymbol = symbolProvider.toSymbol(member)
                val outerType = memberSymbol.rustType()
                val coreType = outerType.stripOuter<RustType.Option>()
                val signature = when (coreType) {
                    is RustType.String -> "(mut self, inp: impl Into<String>) -> Self"
                    is RustType.Box -> "(mut self, inp: impl Into<${coreType.render()}>) -> Self"
                    else -> "(mut self, inp: ${coreType.render()}) -> Self"
                }
                writer.documentShape(member, model)
                writer.rustBlock("pub fn $memberName$signature") {
                    write("self.$memberName = Some(${builderConverter(coreType)});")
                    write("self")
                }
            }

            buildFn(this)
        }
    }

    abstract fun buildFn(writer: RustWriter)

    /**
     * The core builder of the inner type. If the structure requires a fallible builder, this may use `?` to return
     * errors
     * ```rust
     * SomeStruct {
     *    field: builder.field,
     *    field2: builder.field2,
     *    field3: builder.field3.unwrap_or_default()
     *    field4: builder.field4.ok_or("field4 is required when building SomeStruct")?
     * }
     */
    protected fun coreBuilder(writer: RustWriter) {
        writer.rustBlock("#T", structureSymbol) {
            members.forEach { member ->
                val memberName = symbolProvider.toMemberName(member)
                val memberSymbol = symbolProvider.toSymbol(member)
                val errorWhenMissing = "$memberName is required when building ${structureSymbol.name}"
                val modifier = when {
                    !memberSymbol.isOptional() && memberSymbol.canUseDefault() -> ".unwrap_or_default()"
                    !memberSymbol.isOptional() -> ".ok_or(${errorWhenMissing.dq()})?"
                    else -> ""
                }
                write("$memberName: self.$memberName$modifier,")
            }
        }
    }
}
