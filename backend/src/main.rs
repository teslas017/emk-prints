mod config;
mod error;
mod models;
mod routes;
mod state;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer, middleware, web};
use config::Config;
use state::AppState;
use tracing_subscriber::{EnvFilter, fmt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    fmt().json().with_env_filter(EnvFilter::from_default_env()).init();
    let config = Config::from_env().expect("valid configuration");
    let pool = sqlx::PgPool::connect(&config.database_url).await.expect("database connection");
    sqlx::migrate!().run(&pool).await.expect("database migrations");
    let state = web::Data::new(AppState { pool, config: config.clone() });
    let governor = GovernorConfigBuilder::default().requests_per_second(3).burst_size(20).finish().unwrap();
    let bind = format!("{}:{}", config.host, config.port);

    HttpServer::new(move || {
        let origin = state.config.frontend_origin.clone();
        App::new()
            .app_data(state.clone())
            .app_data(web::JsonConfig::default().limit(32 * 1024))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .wrap(Governor::new(&governor))
            .wrap(Cors::default().allowed_origin(&origin).allowed_methods(vec!["GET","POST","PATCH","DELETE"]).allowed_headers(vec![actix_web::http::header::CONTENT_TYPE, actix_web::http::header::AUTHORIZATION, actix_web::http::header::HeaderName::from_static("x-csrf-token")]).supports_credentials().max_age(3600))
            .configure(routes::configure)
    }).bind(bind)?.run().await
}

