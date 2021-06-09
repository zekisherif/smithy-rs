// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>The request parameters represent the input of a SQL statement over an array of
/// data.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BatchExecuteStatementInputBody<'a> {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub resource_arn: &'a std::option::Option<std::string::String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub secret_arn: &'a std::option::Option<std::string::String>,
    /// <p>The SQL statement to run.</p>
    pub sql: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database.</p>
    pub database: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database schema.</p>
    pub schema: &'a std::option::Option<std::string::String>,
    /// <p>The parameter set for the batch operation.</p>
    /// <p>The SQL statement is executed as many times as the number of parameter sets provided.
    /// To execute a SQL statement with no parameters, use one of the following options:</p>
    /// <ul>
    /// <li>
    /// <p>Specify one or more empty parameter sets.</p>
    /// </li>
    /// <li>
    /// <p>Use the <code>ExecuteStatement</code> operation instead of the <code>BatchExecuteStatement</code> operation.</p>
    /// </li>
    /// </ul>
    /// <note>
    /// <p>Array parameters are not supported.</p>
    /// </note>
    pub parameter_sets:
        &'a std::option::Option<std::vec::Vec<std::vec::Vec<crate::model::SqlParameter>>>,
    /// <p>The identifier of a transaction that was started by using the
    /// <code>BeginTransaction</code> operation. Specify the transaction ID of the
    /// transaction that you want to include the SQL statement in.</p>
    /// <p>If the SQL statement is not part of a transaction, don't set this
    /// parameter.</p>
    pub transaction_id: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for BatchExecuteStatementInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BatchExecuteStatementInputBody");
        formatter.field("resource_arn", &self.resource_arn);
        formatter.field("secret_arn", &self.secret_arn);
        formatter.field("sql", &self.sql);
        formatter.field("database", &self.database);
        formatter.field("schema", &self.schema);
        formatter.field("parameter_sets", &self.parameter_sets);
        formatter.field("transaction_id", &self.transaction_id);
        formatter.finish()
    }
}

/// <p>The request parameters represent the input of a request to start a SQL
/// transaction.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BeginTransactionInputBody<'a> {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub resource_arn: &'a std::option::Option<std::string::String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub secret_arn: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database.</p>
    pub database: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database schema.</p>
    pub schema: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for BeginTransactionInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BeginTransactionInputBody");
        formatter.field("resource_arn", &self.resource_arn);
        formatter.field("secret_arn", &self.secret_arn);
        formatter.field("database", &self.database);
        formatter.field("schema", &self.schema);
        formatter.finish()
    }
}

/// <p>The request parameters represent the input of a commit transaction request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CommitTransactionInputBody<'a> {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub resource_arn: &'a std::option::Option<std::string::String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub secret_arn: &'a std::option::Option<std::string::String>,
    /// <p>The identifier of the transaction to end and commit.</p>
    pub transaction_id: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for CommitTransactionInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CommitTransactionInputBody");
        formatter.field("resource_arn", &self.resource_arn);
        formatter.field("secret_arn", &self.secret_arn);
        formatter.field("transaction_id", &self.transaction_id);
        formatter.finish()
    }
}

/// <p>The request parameters represent the input of a request to run one or more SQL
/// statements.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ExecuteSqlInputBody<'a> {
    /// <p>The ARN of the Aurora Serverless DB cluster.</p>
    pub db_cluster_or_instance_arn: &'a std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the secret that enables access to the DB cluster.</p>
    pub aws_secret_store_arn: &'a std::option::Option<std::string::String>,
    /// <p>One or more SQL statements to run on the DB cluster.</p>
    /// <p>You can separate SQL statements from each other with a semicolon (;). Any valid SQL
    /// statement is permitted, including data definition, data manipulation, and commit
    /// statements. </p>
    pub sql_statements: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database.</p>
    pub database: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database schema.</p>
    pub schema: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for ExecuteSqlInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExecuteSqlInputBody");
        formatter.field(
            "db_cluster_or_instance_arn",
            &self.db_cluster_or_instance_arn,
        );
        formatter.field("aws_secret_store_arn", &self.aws_secret_store_arn);
        formatter.field("sql_statements", &self.sql_statements);
        formatter.field("database", &self.database);
        formatter.field("schema", &self.schema);
        formatter.finish()
    }
}

/// <p>The request parameters represent the input of a request to run a SQL statement against
/// a database.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ExecuteStatementInputBody<'a> {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub resource_arn: &'a std::option::Option<std::string::String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub secret_arn: &'a std::option::Option<std::string::String>,
    /// <p>The SQL statement to run.</p>
    pub sql: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database.</p>
    pub database: &'a std::option::Option<std::string::String>,
    /// <p>The name of the database schema.</p>
    /// <note>
    /// <p>Currently, the <code>schema</code> parameter isn't supported.</p>
    /// </note>
    pub schema: &'a std::option::Option<std::string::String>,
    /// <p>The parameters for the SQL statement.</p>
    /// <note>
    /// <p>Array parameters are not supported.</p>
    /// </note>
    pub parameters: &'a std::option::Option<std::vec::Vec<crate::model::SqlParameter>>,
    /// <p>The identifier of a transaction that was started by using the
    /// <code>BeginTransaction</code> operation. Specify the transaction ID of the
    /// transaction that you want to include the SQL statement in.</p>
    /// <p>If the SQL statement is not part of a transaction, don't set this parameter.</p>
    pub transaction_id: &'a std::option::Option<std::string::String>,
    /// <p>A value that indicates whether to include metadata in the results.</p>
    pub include_result_metadata: &'a bool,
    /// <p>A value that indicates whether to continue running the statement after
    /// the call times out. By default, the statement stops running when the call
    /// times out.</p>
    /// <important>
    /// <p>For DDL statements, we recommend continuing to run the statement after
    /// the call times out. When a DDL statement terminates before it is finished
    /// running, it can result in errors and possibly corrupted data structures.</p>
    /// </important>
    pub continue_after_timeout: &'a bool,
    /// <p>Options that control how the result set is returned.</p>
    pub result_set_options: &'a std::option::Option<crate::model::ResultSetOptions>,
}
impl<'a> std::fmt::Debug for ExecuteStatementInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExecuteStatementInputBody");
        formatter.field("resource_arn", &self.resource_arn);
        formatter.field("secret_arn", &self.secret_arn);
        formatter.field("sql", &self.sql);
        formatter.field("database", &self.database);
        formatter.field("schema", &self.schema);
        formatter.field("parameters", &self.parameters);
        formatter.field("transaction_id", &self.transaction_id);
        formatter.field("include_result_metadata", &self.include_result_metadata);
        formatter.field("continue_after_timeout", &self.continue_after_timeout);
        formatter.field("result_set_options", &self.result_set_options);
        formatter.finish()
    }
}

/// <p>The request parameters represent the input of a request to perform a rollback of a
/// transaction.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RollbackTransactionInputBody<'a> {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub resource_arn: &'a std::option::Option<std::string::String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub secret_arn: &'a std::option::Option<std::string::String>,
    /// <p>The identifier of the transaction to roll back.</p>
    pub transaction_id: &'a std::option::Option<std::string::String>,
}
impl<'a> std::fmt::Debug for RollbackTransactionInputBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RollbackTransactionInputBody");
        formatter.field("resource_arn", &self.resource_arn);
        formatter.field("secret_arn", &self.secret_arn);
        formatter.field("transaction_id", &self.transaction_id);
        formatter.finish()
    }
}

/// <p>The response elements represent the output of a SQL statement over an array of
/// data.</p>
#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct BatchExecuteStatementOutputBody {
    /// <p>The execution results of each batch entry.</p>
    #[serde(rename = "updateResults")]
    #[serde(default)]
    pub update_results: std::option::Option<std::vec::Vec<crate::model::UpdateResult>>,
}
impl std::fmt::Debug for BatchExecuteStatementOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BatchExecuteStatementOutputBody");
        formatter.field("update_results", &self.update_results);
        formatter.finish()
    }
}

/// <p>The response elements represent the output of a request to start a SQL
/// transaction.</p>
#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct BeginTransactionOutputBody {
    /// <p>The transaction ID of the transaction started by the call.</p>
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for BeginTransactionOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BeginTransactionOutputBody");
        formatter.field("transaction_id", &self.transaction_id);
        formatter.finish()
    }
}

/// <p>The response elements represent the output of a commit transaction request.</p>
#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct CommitTransactionOutputBody {
    /// <p>The status of the commit operation.</p>
    #[serde(rename = "transactionStatus")]
    #[serde(default)]
    pub transaction_status: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CommitTransactionOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CommitTransactionOutputBody");
        formatter.field("transaction_status", &self.transaction_status);
        formatter.finish()
    }
}

/// <p>The response elements represent the output of a request to run one or more SQL
/// statements.</p>
#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct ExecuteSqlOutputBody {
    /// <p>The results of the SQL statement or statements.</p>
    #[serde(rename = "sqlStatementResults")]
    #[serde(default)]
    pub sql_statement_results: std::option::Option<std::vec::Vec<crate::model::SqlStatementResult>>,
}
impl std::fmt::Debug for ExecuteSqlOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExecuteSqlOutputBody");
        formatter.field("sql_statement_results", &self.sql_statement_results);
        formatter.finish()
    }
}

/// <p>The response elements represent the output of a request to run a SQL statement against
/// a database.</p>
#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct ExecuteStatementOutputBody {
    /// <p>The records returned by the SQL statement.</p>
    #[serde(rename = "records")]
    #[serde(default)]
    pub records: std::option::Option<std::vec::Vec<std::vec::Vec<crate::model::Field>>>,
    /// <p>Metadata for the columns included in the results.</p>
    #[serde(rename = "columnMetadata")]
    #[serde(default)]
    pub column_metadata: std::option::Option<std::vec::Vec<crate::model::ColumnMetadata>>,
    /// <p>The number of records updated by the request.</p>
    #[serde(rename = "numberOfRecordsUpdated")]
    #[serde(default)]
    pub number_of_records_updated: i64,
    /// <p>Values for fields generated during the request.</p>
    /// <note>
    /// <p>The <code>generatedFields</code> data isn't supported by Aurora PostgreSQL.
    /// To get the values of generated fields, use the <code>RETURNING</code> clause. For
    /// more information, see <a href="https://www.postgresql.org/docs/10/dml-returning.html">Returning Data From
    /// Modified Rows</a> in the PostgreSQL documentation.</p>
    /// </note>
    #[serde(rename = "generatedFields")]
    #[serde(default)]
    pub generated_fields: std::option::Option<std::vec::Vec<crate::model::Field>>,
}
impl std::fmt::Debug for ExecuteStatementOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExecuteStatementOutputBody");
        formatter.field("records", &self.records);
        formatter.field("column_metadata", &self.column_metadata);
        formatter.field("number_of_records_updated", &self.number_of_records_updated);
        formatter.field("generated_fields", &self.generated_fields);
        formatter.finish()
    }
}

/// <p>The response elements represent the output of a request to perform a rollback of a
/// transaction.</p>
#[non_exhaustive]
#[derive(std::default::Default, serde::Deserialize, std::clone::Clone, std::cmp::PartialEq)]
pub struct RollbackTransactionOutputBody {
    /// <p>The status of the rollback operation.</p>
    #[serde(rename = "transactionStatus")]
    #[serde(default)]
    pub transaction_status: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for RollbackTransactionOutputBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RollbackTransactionOutputBody");
        formatter.field("transaction_status", &self.transaction_status);
        formatter.finish()
    }
}
