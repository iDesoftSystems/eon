use crate::contract::PersistenceClient;

/// A trait for commands that can be executed directly against a persistence client.
///
/// This trait allows defining self-contained commands that encapsulate both the data
/// and the logic required to execute them.
pub trait ExecutableCommand<TClient: PersistenceClient + ?Sized> {
    /// The type of the result produced by executing the command.
    type Output;

    /// Executes the command using the provided persistence client.
    ///
    /// # Arguments
    ///
    /// * `client` - The persistence client to use for data access.
    ///
    /// # Returns
    ///
    /// * `Self::Output` - The result of the command execution.
    fn execute(self, client: &TClient) -> impl Future<Output = Self::Output> + Send;
}
