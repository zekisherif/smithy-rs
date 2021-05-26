// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendCommandOutput {
    /// <p>Contains the details of the started session that includes a session token. This
    /// <code>SessionToken</code> is required for every subsequent command that is issued during
    /// the current session.</p>
    pub start_session: std::option::Option<crate::model::StartSessionResult>,
    /// <p>Contains the details of the started transaction.</p>
    pub start_transaction: std::option::Option<crate::model::StartTransactionResult>,
    /// <p>Contains the details of the ended session.</p>
    pub end_session: std::option::Option<crate::model::EndSessionResult>,
    /// <p>Contains the details of the committed transaction.</p>
    pub commit_transaction: std::option::Option<crate::model::CommitTransactionResult>,
    /// <p>Contains the details of the aborted transaction.</p>
    pub abort_transaction: std::option::Option<crate::model::AbortTransactionResult>,
    /// <p>Contains the details of the executed statement.</p>
    pub execute_statement: std::option::Option<crate::model::ExecuteStatementResult>,
    /// <p>Contains the details of the fetched page.</p>
    pub fetch_page: std::option::Option<crate::model::FetchPageResult>,
}
impl std::fmt::Debug for SendCommandOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendCommandOutput");
        formatter.field("start_session", &self.start_session);
        formatter.field("start_transaction", &self.start_transaction);
        formatter.field("end_session", &self.end_session);
        formatter.field("commit_transaction", &self.commit_transaction);
        formatter.field("abort_transaction", &self.abort_transaction);
        formatter.field("execute_statement", &self.execute_statement);
        formatter.field("fetch_page", &self.fetch_page);
        formatter.finish()
    }
}
/// See [`SendCommandOutput`](crate::output::SendCommandOutput)
pub mod send_command_output {
    /// A builder for [`SendCommandOutput`](crate::output::SendCommandOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) start_session: std::option::Option<crate::model::StartSessionResult>,
        pub(crate) start_transaction: std::option::Option<crate::model::StartTransactionResult>,
        pub(crate) end_session: std::option::Option<crate::model::EndSessionResult>,
        pub(crate) commit_transaction: std::option::Option<crate::model::CommitTransactionResult>,
        pub(crate) abort_transaction: std::option::Option<crate::model::AbortTransactionResult>,
        pub(crate) execute_statement: std::option::Option<crate::model::ExecuteStatementResult>,
        pub(crate) fetch_page: std::option::Option<crate::model::FetchPageResult>,
    }
    impl Builder {
        /// <p>Contains the details of the started session that includes a session token. This
        /// <code>SessionToken</code> is required for every subsequent command that is issued during
        /// the current session.</p>
        pub fn start_session(mut self, inp: crate::model::StartSessionResult) -> Self {
            self.start_session = Some(inp);
            self
        }
        pub fn set_start_session(
            mut self,
            inp: std::option::Option<crate::model::StartSessionResult>,
        ) -> Self {
            self.start_session = inp;
            self
        }
        /// <p>Contains the details of the started transaction.</p>
        pub fn start_transaction(mut self, inp: crate::model::StartTransactionResult) -> Self {
            self.start_transaction = Some(inp);
            self
        }
        pub fn set_start_transaction(
            mut self,
            inp: std::option::Option<crate::model::StartTransactionResult>,
        ) -> Self {
            self.start_transaction = inp;
            self
        }
        /// <p>Contains the details of the ended session.</p>
        pub fn end_session(mut self, inp: crate::model::EndSessionResult) -> Self {
            self.end_session = Some(inp);
            self
        }
        pub fn set_end_session(
            mut self,
            inp: std::option::Option<crate::model::EndSessionResult>,
        ) -> Self {
            self.end_session = inp;
            self
        }
        /// <p>Contains the details of the committed transaction.</p>
        pub fn commit_transaction(mut self, inp: crate::model::CommitTransactionResult) -> Self {
            self.commit_transaction = Some(inp);
            self
        }
        pub fn set_commit_transaction(
            mut self,
            inp: std::option::Option<crate::model::CommitTransactionResult>,
        ) -> Self {
            self.commit_transaction = inp;
            self
        }
        /// <p>Contains the details of the aborted transaction.</p>
        pub fn abort_transaction(mut self, inp: crate::model::AbortTransactionResult) -> Self {
            self.abort_transaction = Some(inp);
            self
        }
        pub fn set_abort_transaction(
            mut self,
            inp: std::option::Option<crate::model::AbortTransactionResult>,
        ) -> Self {
            self.abort_transaction = inp;
            self
        }
        /// <p>Contains the details of the executed statement.</p>
        pub fn execute_statement(mut self, inp: crate::model::ExecuteStatementResult) -> Self {
            self.execute_statement = Some(inp);
            self
        }
        pub fn set_execute_statement(
            mut self,
            inp: std::option::Option<crate::model::ExecuteStatementResult>,
        ) -> Self {
            self.execute_statement = inp;
            self
        }
        /// <p>Contains the details of the fetched page.</p>
        pub fn fetch_page(mut self, inp: crate::model::FetchPageResult) -> Self {
            self.fetch_page = Some(inp);
            self
        }
        pub fn set_fetch_page(
            mut self,
            inp: std::option::Option<crate::model::FetchPageResult>,
        ) -> Self {
            self.fetch_page = inp;
            self
        }
        /// Consumes the builder and constructs a [`SendCommandOutput`](crate::output::SendCommandOutput)
        pub fn build(self) -> crate::output::SendCommandOutput {
            crate::output::SendCommandOutput {
                start_session: self.start_session,
                start_transaction: self.start_transaction,
                end_session: self.end_session,
                commit_transaction: self.commit_transaction,
                abort_transaction: self.abort_transaction,
                execute_statement: self.execute_statement,
                fetch_page: self.fetch_page,
            }
        }
    }
}
impl SendCommandOutput {
    /// Creates a new builder-style object to manufacture [`SendCommandOutput`](crate::output::SendCommandOutput)
    pub fn builder() -> crate::output::send_command_output::Builder {
        crate::output::send_command_output::Builder::default()
    }
}
