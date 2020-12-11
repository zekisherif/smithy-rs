package software.amazon.smithy.rust.codegen.smithy.generators

import org.junit.jupiter.api.Test
import software.amazon.smithy.codegen.core.Symbol
import software.amazon.smithy.codegen.core.SymbolProvider
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.Shape
import software.amazon.smithy.rust.codegen.generators.StructureGeneratorTest
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.smithy.RustSymbolProvider
import software.amazon.smithy.rust.codegen.smithy.SymbolVisitorConfig
import software.amazon.smithy.rust.codegen.smithy.canUseDefault
import software.amazon.smithy.rust.testutil.compileAndTest
import software.amazon.smithy.rust.testutil.testSymbolProvider

internal class ModelBuilderGeneratorTest {
    private val model = StructureGeneratorTest.model
    private val inner = StructureGeneratorTest.inner
    private val struct = StructureGeneratorTest.struct

    @Test
    fun `generate builders`() {
        val provider = testSymbolProvider(model)
        val writer = RustWriter.forModule("model")
        val innerGenerator = StructureGenerator(model, provider, writer, inner)
        val generator = StructureGenerator(model, provider, writer, struct)
        val builderGenerator = ModelBuilderGenerator(model, provider, struct)
        generator.render()
        innerGenerator.render()
        builderGenerator.render(writer)
        writer.implBlock(struct, provider) {
            builderGenerator.renderConvenienceMethod(this)
        }
        writer.compileAndTest(
            """
            let my_struct = MyStruct::builder().byte_value(4).foo("hello!").build();
            assert_eq!(my_struct.foo.unwrap(), "hello!");
            assert_eq!(my_struct.bar, 0);
        """
        )
    }

    @Test
    fun `generate fallible builders`() {
        val baseProvider: SymbolProvider = testSymbolProvider(StructureGeneratorTest.model)
        val provider =
            object : RustSymbolProvider {
                override fun config(): SymbolVisitorConfig {
                    TODO("Not yet implemented")
                }

                override fun toSymbol(shape: Shape?): Symbol {
                    return baseProvider.toSymbol(shape).toBuilder().canUseDefault(false).build()
                }

                override fun toMemberName(shape: MemberShape?): String {
                    return baseProvider.toMemberName(shape)
                }
            }
        val writer = RustWriter.forModule("model")
        val innerGenerator = StructureGenerator(
            StructureGeneratorTest.model, provider, writer,
            StructureGeneratorTest.inner
        )
        val generator = StructureGenerator(
            StructureGeneratorTest.model, provider, writer,
            StructureGeneratorTest.struct
        )
        generator.render()
        innerGenerator.render()
        val builderGenerator = ModelBuilderGenerator(model, provider, struct)
        builderGenerator.render(writer)
        writer.implBlock(struct, provider) {
            builderGenerator.renderConvenienceMethod(this)
        }
        writer.compileAndTest(
            """
            let my_struct = MyStruct::builder().byte_value(4).foo("hello!").bar(0).build().expect("required field was not provided");
            assert_eq!(my_struct.foo.unwrap(), "hello!");
            assert_eq!(my_struct.bar, 0);
        """
        )
    }
}
