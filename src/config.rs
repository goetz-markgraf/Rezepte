use std::env;
use std::sync::Mutex;

// Mutex to serialize tests that modify environment variables
static ENV_LOCK: Mutex<()> = Mutex::new(());

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        // For tests, use TEST_DATABASE_URL if set, otherwise fall back to DATABASE_URL
        let database_url = env::var("TEST_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .unwrap_or_else(|_| "sqlite:data/recipes.db".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        Config { database_url, port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_uses_defaults_when_no_env_vars() {
        let _guard = ENV_LOCK.lock().unwrap();
        
        // Clear environment variables for this test
        env::remove_var("TEST_DATABASE_URL");
        env::remove_var("DATABASE_URL");
        env::remove_var("PORT");

        let config = Config::from_env();

        assert_eq!(config.database_url, "sqlite:data/recipes.db");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn config_uses_env_vars_when_set() {
        let _guard = ENV_LOCK.lock().unwrap();
        
        env::remove_var("TEST_DATABASE_URL");
        env::set_var("DATABASE_URL", "/custom/path.db");
        env::set_var("PORT", "3000");

        let config = Config::from_env();

        assert_eq!(config.database_url, "/custom/path.db");
        assert_eq!(config.port, 3000);

        // Cleanup
        env::remove_var("DATABASE_URL");
        env::remove_var("PORT");
    }
}
