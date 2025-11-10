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

async fn handler() -> Html<String> {
    let config_file_path: Option<String> = std::env::var("LORES_APP_CONFIG_FILE_PATH").ok();
    let config_text: Option<String> = match config_file_path {
        Some(path) => {
            println!("Looking for config file at: {:?}", path);
            let config_text = load_config_text(&path);
            if config_text.is_none() {
                println!(" -- File not found");
            }
            config_text
        }
        None => {
            println!("No config file path provided");
            None
        }
    };

    Html(format!(
        r#"
    <html>
        <head>
            <title>Config Example</title>
        </head>
        <body>
            <h1>LoRes App: Config Example</h1>
            <p>This application demonstrates configuration of a LoRes App. Set the configuration as a Node Steward using LoRes Node. That config will be provided to this app as a JSON file, and once
            you have created it, it will be displayed below:</p>

            <code>
                <pre>{}</pre>
            </code>
        </body>
    </html>
    "#,
        { config_text.unwrap_or_else(|| "No config file found or configured".to_string()) }
    ))
}

fn load_config_text(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
