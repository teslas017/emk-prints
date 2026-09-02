use crate::config::Config;
#[derive(Clone)] pub struct AppState{pub pool:sqlx::PgPool,pub config:Config}

