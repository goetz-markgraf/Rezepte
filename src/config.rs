use std::env;

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
// Mutex to serialize tests that modify environment variables
static ENV_LOCK: Mutex<()> = Mutex::new(());

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub vision_api_url: Option<String>,
    pub vision_api_key: Option<String>,
    pub vision_model: String,
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
        let vision_api_url = env::var("VISION_API_URL").ok();
        let vision_api_key = env::var("VISION_API_KEY").ok();
        let vision_model = env::var("VISION_MODEL")
            .unwrap_or_else(|_| "gpt-4o".to_string());

        Config {
            database_url,
            port,
            vision_api_url,
            vision_api_key,
            vision_model,
        }
    }

    pub fn vision_enabled(&self) -> bool {
        self.vision_api_url.is_some() && self.vision_api_key.is_some()
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
        env::remove_var("VISION_API_URL");
        env::remove_var("VISION_API_KEY");
        env::remove_var("VISION_MODEL");

        let config = Config::from_env();

        assert_eq!(config.database_url, "sqlite:data/recipes.db");
        assert_eq!(config.port, 8080);
        assert!(config.vision_api_url.is_none());
        assert!(config.vision_api_key.is_none());
        assert_eq!(config.vision_model, "gpt-4o");
        assert!(!config.vision_enabled());
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

    #[test]
    fn vision_enabled_when_both_vars_set() {
        let _guard = ENV_LOCK.lock().unwrap();

        env::set_var("VISION_API_URL", "https://api.openai.com/v1");
        env::set_var("VISION_API_KEY", "sk-test");
        env::remove_var("VISION_MODEL");

        let config = Config::from_env();

        assert!(config.vision_enabled());
        assert_eq!(config.vision_model, "gpt-4o");

        env::remove_var("VISION_API_URL");
        env::remove_var("VISION_API_KEY");
    }

    #[test]
    fn vision_disabled_when_only_url_set() {
        let _guard = ENV_LOCK.lock().unwrap();

        env::set_var("VISION_API_URL", "https://api.openai.com/v1");
        env::remove_var("VISION_API_KEY");

        let config = Config::from_env();

        assert!(!config.vision_enabled());

        env::remove_var("VISION_API_URL");
    }

    #[test]
    fn vision_model_can_be_overridden() {
        let _guard = ENV_LOCK.lock().unwrap();

        env::set_var("VISION_MODEL", "gpt-4-turbo");

        let config = Config::from_env();

        assert_eq!(config.vision_model, "gpt-4-turbo");

        env::remove_var("VISION_MODEL");
    }
}
