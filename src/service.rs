use async_trait::async_trait;

#[async_trait]
pub trait Service<Request>: Send + Sync + 'static {
    type Response;
    type Error;

    async fn call(&self, req: Request) -> Result<Self::Response, Self::Error>
    where
        Request: 'async_trait;
}
