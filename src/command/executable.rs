pub trait PersistenceClient: Send + Sync + 'static {}

pub trait ExecutableCommand<TClient: PersistenceClient + ?Sized> {
    type Output;
    fn execute(self, client: &TClient) -> impl Future<Output = Self::Output> + Send;
}
