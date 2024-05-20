use axum::routing::get;

#[tokio::main]
async fn main() {
    let router = rspc::Router::<()>::new()
        .query("version", |t| t(|_, ()| "1.0.0"))
        .build()
        .arced();

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest("/rspc", rspc_axum::endpoint(router, || ()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
