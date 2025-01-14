// src/errors/mod.rs
pub mod api;
pub mod server;

pub use api::ApiError;
pub use server::ServerError;
