// src/errors/mod.rs
pub mod api;
pub mod server;

pub use api::ApiError;
pub use server::ServerError;
pub fn map_to_io_error<E: std::fmt::Display>(err: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, format!("{}", err))
}
