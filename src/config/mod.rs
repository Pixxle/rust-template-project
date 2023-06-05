use config;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub database_url: String,
    pub username: String,
    pub password: String,

    #[serde(skip)]
    pub environment: Environment,
}

pub fn parse() -> Configuration {
    println!("Parsing config");

    let env = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .separator("_")
                .ignore_empty(true),
        )
        .build();

    println!("{:?}", env);

    let env = match env {
        Ok(env) => env.get_string("ENV").unwrap_or({
            println!("Found APP prefix but no ENV variable. Defaulting to development");
            "development".to_string()
        }),
        Err(_) => {
            println!("No APP prefix found. Defaulting to development");
            "development".to_string()
        }
    };

    let configuration = config::Config::builder()
        .add_source(config::File::with_name(
            format!("src/config/{}.json", env).as_str(),
        ))
        .build()
        .unwrap();

    let mut configuration: Configuration = configuration.try_deserialize().unwrap();

    configuration.environment = match env.as_str() {
        "development" => Environment::Development,
        "production" => Environment::Production,
        _ => panic!("Invalid environment"),
    };

    configuration
}
