use axum::Router;
use std::io;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> io::Result<()> {
    // serve static files from assets folder
    let serve_dir = ServeDir::new("assets");

    // setup route to root
    let app = Router::new().fallback_service(serve_dir);

    // TODO set route via commanline or choose random port
    // TODO figure out how check if a port is already being used
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
