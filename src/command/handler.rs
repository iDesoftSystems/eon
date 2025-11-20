/// A trait for handlers that process specific commands.
///
/// This trait decouples the command data from the execution logic, allowing for
/// separate handlers to be registered for different command types.
pub trait CommandHandler<TCommand> {
    /// The type of the result produced by handling the command.
    type Output;

    /// Handles the given command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to handle.
    ///
    /// # Returns
    ///
    /// * `Self::Output` - The result of the command handling.
    fn handle(&self, command: &TCommand) -> impl Future<Output = Self::Output>;
}
