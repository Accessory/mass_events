mod app_state;
mod configuration;
mod controller;
mod entities;
mod open_api;
mod service;
mod utils;

use crate::app_state::init_scheduler;
use crate::controller::scheduler_controller;
use crate::service::scheduler_service::SchedulerService;
use axum::{Router, response::Redirect, routing::get};
use clap::Parser;
use configuration::Configuration;
use mass_events_process_runner_client::ProcessRunnerClient;
use serde_merge::omerge;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use std::str::FromStr;
use std::{fs::File, io::BufReader, sync::Arc, time::Duration};

use crate::open_api::ApiDoc;
use crate::{app_state::AppState, configuration::FileConfiguration};

// #[tokio::main]
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

    let app_state = Arc::new(RwLock::new(AppState::new_with(pool.clone()).await));

    let scheduler_service: Arc<SchedulerService> = Arc::new(SchedulerService {
        state: app_state.clone(),
    });

    let process_runner_client = Arc::new(ProcessRunnerClient::new("http://localhost:8456"));

    init_scheduler(
        app_state,
        scheduler_service.clone(),
        process_runner_client.clone(),
    )
    .await;

    // build our application with some routes
    let app = Router::new()
        .nest(
            "/scheduler",
            scheduler_controller::new_router(scheduler_service),
        )
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
            serde_yaml::from_reader(reader).expect("Config file could not be parsed");
        configuration = omerge(configuration, config).expect("Failed to merge configs");
    }
    println!("{}", &configuration);
    configuration
}

async fn redirect_to_openapi() -> Redirect {
    Redirect::permanent("/swagger-ui/")
}
