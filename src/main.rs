pub type Result<T, E = anyhow::Error> = core::result::Result<T, E>;

mod handlers;
mod routes;
mod tracing;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();
    let router = routes::create_router();
    let address = "0.0.0.0:8888".parse()?;
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}
