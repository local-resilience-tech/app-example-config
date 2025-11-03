use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let port = std::env::var("HTTP_PORT").unwrap_or_else(|_| "8200".to_string());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    println!("Handling request");
    Html("<h1>Hello, World!</h1>")
}
