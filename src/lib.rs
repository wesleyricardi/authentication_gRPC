use sqlx::{Pool, Postgres};
pub mod controllers;
pub mod database;
pub mod dtos;
pub mod error;
pub mod models;
pub mod repositories;
pub mod rpc;
pub mod security;
pub mod services;
pub mod utils;

pub struct AppState {
    db_pg_pool: Pool<Postgres>,
}
