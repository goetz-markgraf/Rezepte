pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod routes;
pub mod templates;

pub use config::Config;
pub use db::create_pool;
pub use routes::create_router;
