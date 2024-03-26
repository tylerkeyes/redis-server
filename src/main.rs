use dotenv::dotenv;
use log::info;
use std::env;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
//use tokio::signal;

mod connection;
mod controllers;
mod data;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Redis server starting...");

    let app_development = env::var("APP_ENVIRONMENT").unwrap_or("development".to_string());
    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("6379".to_string());
    info!("Server running on {}:{}...", app_host, app_port);
    let bind_address = app_host + ":" + &app_port;

    match app_development.as_str() {
        "development" => {
            info!("running in development mode");
        }
        "production" => {
            info!("running in production mode");
        }
        _ => {
            info!("running in development mode");
        }
    }

    let listener = TcpListener::bind(&bind_address).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {}

/*
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Redis server starting...");

    let app_development = env::var("APP_ENVIRONMENT").unwrap_or("development".to_string());
    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("6379".to_string());

    info!("Server running on {}:{}...", app_host, app_port);

    match app_development.as_str() {
        "development" => {
            info!("running in development mode");
        }
        "production" => {
            info!("running in production mode");
        }
        _ => {
            info!("running in development mode");
        }
    }

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/:input", get(redis_input))
        .merge(service_controller::router().layer(cors));

    let bind_address = app_host + ":" + &app_port;
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn redis_input(Path(input): Path<String>) -> Result<Response<axum::body::Body>, Infallible> {
    println!("default handler with input: {}", input);
    let res = Response::new(Body::new("response".to_string()));
    Ok::<_, Infallible>(res)
}
*/

/*
#[allow(dead_code)]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, shutting down");
}
*/
