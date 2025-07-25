pub trait PersistenceClient: Send + Sync + 'static {}

pub trait ExecutableCommand<TClient: PersistenceClient + ?Sized> {
    type Output;
    fn execute<'a>(self, client: &'a TClient) -> impl Future<Output = Self::Output> + Send + 'a;
}
