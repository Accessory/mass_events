use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(long, default_value = "127.0.0.1", env = "MASS_EVENTS_IP")]
    pub ip: String,
    #[arg(long, default_value = "8456", env = "MASS_EVENTS_PORT")]
    pub port: u16,
    #[arg(long, default_value = "postgres://localhost:5432/RustRobot", env)]
    pub database_url: String,
    #[arg(long)]
    pub config_file: Option<String>,
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ip: {}", &self.ip)?;
        writeln!(f, "port: {}", &self.port)?;
        writeln!(f, "database_url: {}", &self.database_url)?;
        writeln!(
            f,
            "config_file: {}",
            &self.config_file.as_ref().unwrap_or(&String::from(""))
        )
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct FileConfiguration {
    #[arg(long, default_value = "127.0.0.1")]
    pub ip: Option<String>,
    #[arg(long, default_value = "500")]
    pub port: Option<u16>,
    #[arg(long, default_value = "postgres://localhost:5432/Robot")]
    pub database_url: Option<String>,
}
