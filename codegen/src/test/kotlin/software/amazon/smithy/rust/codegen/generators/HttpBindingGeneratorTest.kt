/*
 * Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *  http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 */

package software.amazon.smithy.rust.codegen.generators

import io.kotest.matchers.shouldBe
import org.junit.jupiter.api.Test
import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.model.shapes.ShapeId
import software.amazon.smithy.model.traits.HttpTrait
import software.amazon.smithy.rust.codegen.lang.RustWriter
import software.amazon.smithy.rust.codegen.smithy.generators.OperationGenerator
import software.amazon.smithy.rust.codegen.smithy.generators.uriFormatString
import software.amazon.smithy.rust.codegen.util.dq
import software.amazon.smithy.rust.testutil.TestRuntimeConfig
import software.amazon.smithy.rust.testutil.asSmithy
import software.amazon.smithy.rust.testutil.shouldCompile
import software.amazon.smithy.rust.testutil.testSymbolProvider

class HttpBindingGeneratorTest {
    private val model = """
            namespace smithy.example

            @idempotent
            @http(method: "PUT", uri: "/{bucketName}/{key}", code: 200)
            operation PutObject {
                input: PutObjectRequest
            }
            
            list Extras {
                member: Integer
            }
            
            list Dates {
                member: Timestamp
            }
            
            @mediaType("video/quicktime")
            string Video

            structure PutObjectRequest {
                // Sent in the URI label named "key".
                @required
                @httpLabel
                key: Timestamp,

                // Sent in the URI label named "bucketName".
                @required
                @httpLabel
                bucketName: String,

                // Sent in the X-Dates header
                @httpHeader("X-Dates")
                dateHeaderList: Dates,
                
                @httpHeader("X-Ints")
                intList: Extras,
                
                @httpHeader("X-MediaType")
                mediaType: Video,

                // Sent in the query string as paramName
                @httpQuery("paramName")
                someValue: String,
                
                @httpQuery("hello")
                extras: Extras,

                // Sent in the body
                data: Blob,

                // Sent in the body
                additional: String,
            }
        """.asSmithy()

    private val operationShape = model.expectShape(ShapeId.from("smithy.example#PutObject"), OperationShape::class.java)
    private val httpTrait = operationShape.expectTrait(HttpTrait::class.java)

    private fun renderOperation(writer: RustWriter) {
        OperationGenerator(model, testSymbolProvider(model), TestRuntimeConfig, writer, operationShape).render()
    }

    @Test
    fun `produce correct uri format strings`() {
        httpTrait.uriFormatString() shouldBe ("/{bucketName}/{key}".dq())
    }

    // TODO: when we generate builders, use them to clean up these tests; 1h
    @Test
    fun `generate uris`() {
        val writer = RustWriter("operation.rs", "operation")
        // currently rendering the operation renders the protocols—I want to separate that at some point.
        renderOperation(writer)
        println(writer.toString())
        writer.shouldCompile(
            """
            let ts = Instant::from_epoch_seconds(10123125);
            let inp = PutObjectInput {
              additional: None,
              bucket_name: "somebucket/ok".to_string(),
              data: None,
              date_header_list: None,
              key: ts.clone(),
              int_list: None,
              extras: Some(vec![0, 1,2,44]),
              some_value: Some("svq!!%&".to_string()),
              media_type: None
            };
            let mut o = String::new();
            inp.uri_base(&mut o);
            assert_eq!(o.as_str(), "/somebucket%2Fok/1970-04-28T03:58:45Z");
            o.clear();
            inp.uri_query(&mut o);
            assert_eq!(o.as_str(), "?paramName=svq!!%25%26&hello=0&hello=1&hello=2&hello=44")
        """.trimIndent()
        )
    }

    @Test
    fun `build http requests`() {
        val writer = RustWriter("operation.rs", "operation")
        renderOperation(writer)
        writer.shouldCompile(
            """
            let ts = Instant::from_epoch_seconds(10123125);
            let inp = PutObjectInput {
              additional: None,
              bucket_name: "buk".to_string(),
              data: None,
              date_header_list: Some(vec![ts.clone()]),
              int_list: Some(vec![0,1,44]),
              key: Instant::from_epoch_seconds(10123125),
              extras: Some(vec![0,1]),
              some_value: Some("qp".to_string()),
              media_type: Some("base64encodethis".to_string()),
            };
            let http_request = inp.build_http_request(::http::Request::builder()).body(()).unwrap();
            assert_eq!(http_request.uri(), "/buk/1970-04-28T03:58:45Z?paramName=qp&hello=0&hello=1");
            assert_eq!(http_request.method(), "PUT");
            let mut date_header = http_request.headers().get_all("X-Dates").iter();
            assert_eq!(date_header.next().unwrap(), "Tue, 28 Apr 1970 03:58:45 GMT");
            assert_eq!(date_header.next(), None);
            
            let int_header = http_request.headers().get_all("X-Ints").iter().map(|hv|hv.to_str().unwrap()).collect::<Vec<_>>();
            assert_eq!(int_header, vec!["0", "1", "44"]);
            
            let base64_header = http_request.headers().get_all("X-MediaType").iter().map(|hv|hv.to_str().unwrap()).collect::<Vec<_>>();
            assert_eq!(base64_header, vec!["YmFzZTY0ZW5jb2RldGhpcw=="]);
        """
        )
    }
}
