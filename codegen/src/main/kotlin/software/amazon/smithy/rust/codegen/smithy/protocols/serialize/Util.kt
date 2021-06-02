/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy.protocols.serialize

import software.amazon.smithy.model.Model
import software.amazon.smithy.model.shapes.BooleanShape
import software.amazon.smithy.model.shapes.DoubleShape
import software.amazon.smithy.model.shapes.FloatShape
import software.amazon.smithy.model.shapes.MemberShape
import software.amazon.smithy.model.shapes.NumberShape
import software.amazon.smithy.rust.codegen.rustlang.RustWriter
import software.amazon.smithy.rust.codegen.rustlang.rustBlock

class SerializerUtil(private val model: Model) {
    fun RustWriter.ignoreZeroValues(shape: MemberShape, value: ValueExpression, inner: RustWriter.() -> Unit) {
        val expr = when (model.expectShape(shape.target)) {
            is FloatShape, is DoubleShape -> "${value.asValue()} != 0.0"
            is NumberShape -> "${value.asValue()} != 0"
            is BooleanShape -> value.asValue()
            else -> null
        }
        if (expr != null) {
            rustBlock("if $expr") {
                inner(this)
            }
        } else {
            rustBlock("") {
                inner(this)
            }
        }
    }
}
