package software.amazon.smithy.rust.codegen.smithy.transformers

import io.kotest.matchers.shouldBe
import org.junit.jupiter.api.Test
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.rust.codegen.smithy.traits.RustBoxTrait
import software.amazon.smithy.rust.codegen.util.lookup
import software.amazon.smithy.rust.testutil.asSmithyModel
import kotlin.streams.toList

internal class RecursiveShapeBoxerTest {
    @Test
    fun `leave non-recursive models unchanged`() {
        val model = """
            namespace com.example
            list BarList {
                member: Bar
            }
            structure Hello {
                bars: BarList
            }

            structure Bar {
                hello: Hello
            }
        """.asSmithyModel()
        RecursiveShapeBoxer.transform(model) shouldBe model
    }

    @Test
    fun `add the box trait to simple recursive shapes`() {
        val model = """
        namespace com.example
        structure Recursive {
            RecursiveStruct: Recursive,
            anotherField: Boolean
        }
        """.asSmithyModel()
        val transformed = RecursiveShapeBoxer.transform(model)
        val member: MemberShape = transformed.lookup("com.example#Recursive\$RecursiveStruct")
        member.expectTrait(RustBoxTrait::class.java)
    }

    @Test
    fun `add the box trait to complex structures`() {
        val model = """
       namespace com.example
       structure Expr {
            left: Atom,
            right: Atom
       }

       union Atom {
            add: Expr,
            sub: Expr,
            literal: Integer,
            more: SecondTree
       }

       structure SecondTree {
            member: Expr,
            otherMember: Atom,
            third: SecondTree
       }
       """.asSmithyModel()
        val transformed = RecursiveShapeBoxer.transform(model)
        val boxed = transformed.shapes().filter { it.hasTrait(RustBoxTrait::class.java) }.toList()
        boxed.map { it.id.toString().removePrefix("com.example#") }.toSet() shouldBe setOf(
            "Atom\$add",
            "Atom\$sub",
            "SecondTree\$third",
            "Atom\$more"
        )
    }
}
