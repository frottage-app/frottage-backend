use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("server running or whatever");
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .fallback_service(ServeFile::new("index.html"))
        .route("/frottage", post(frottage_function));

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn frottage_function(body: String) -> &'static str {
    println!("Lets frottage in the browser now!");
    println!("This is the {}", body);
    "Lets frottage :)"
}
