pub trait CommandHandler<TCommand> {
    type Output;
    fn handle(&self, command: &TCommand) -> impl Future<Output = Self::Output>;
}
