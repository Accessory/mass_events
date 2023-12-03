use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(short, long, default_value = "http://localhost:8456", env)]
    pub mass_events_process_runner_base_url: String,
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "mass_events_process_runner_base_url: {}", &self.mass_events_process_runner_base_url)
    }
}