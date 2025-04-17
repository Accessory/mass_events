mod app_state;
mod configuration;
mod controller;
mod open_api;
mod service;
mod templates;

use crate::controller::{process_controller, queue_controller};
use crate::service::process_service::ProcessService;
use crate::service::queue_service::QueueService;
use axum::{Router, routing::get};
use clap::Parser;
use configuration::Configuration;
use serde_merge::omerge;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tower_http::trace::TraceLayer;
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use std::str::FromStr;
use std::{fs::File, io::BufReader, sync::Arc, time::Duration};

use crate::configuration::FileConfiguration;
use crate::open_api::{ApiDoc, redirect_to_openapi};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config: Configuration = init_configuration();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let db_connection_str = config.database_url.as_str();

    // Disable Set connection string and disable logging
    let database_connection_options = PgConnectOptions::from_str(db_connection_str)
        .unwrap()
        .disable_statement_logging()
        .clone();

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(database_connection_options)
        .await
        .expect("Can't connect to database");

    // let _app_state = Arc::new(RwLock::new(AppState::new_with(pool.clone()).await));

    let queue_service: Arc<QueueService> = Arc::new(QueueService::new(pool.clone()));
    let process_service: Arc<ProcessService> =
        Arc::new(ProcessService::new(pool.clone(), queue_service.clone()));

    // build our application with some routes
    let app = Router::new()
        .nest("/queues", queue_controller::new_router(queue_service))
        .nest("/process", process_controller::new_router(process_service))
        .route("/", get(redirect_to_openapi))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http());

    // run it with hyper
    let ip = if config.ip == "localhost" {
        "127.0.0.1"
    } else {
        &config.ip
    };
    let addr = format!("{}:{}", ip, config.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn init_configuration() -> Configuration {
    let mut configuration = Configuration::parse();
    if configuration.config_file.is_some() {
        let cf = configuration.config_file.as_ref().unwrap();
        let file = File::open(cf).expect("Failed to open the configuration File at: {cf}");
        let reader = BufReader::new(file);
        let config: FileConfiguration =
            serde_yml::from_reader(reader).expect("Config file could not be parsed");
        configuration = omerge(configuration, config).expect("Failed to merge configs");
    }
    println!("{}", &configuration);
    configuration
}
