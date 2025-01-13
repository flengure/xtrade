// src/api/mod.rs
pub mod bots;
pub mod listeners;

use reqwest::blocking::Client;
use reqwest::Method;

pub use bots::clients::{
    add_bot, add_listener, delete_bot, delete_listener, fetch_bot, fetch_bots, fetch_listener,
    fetch_listeners, update_bot, update_listener,
};

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(bots::endpoints::get_bots);
    cfg.service(bots::endpoints::get_bot_by_id);
    cfg.service(bots::endpoints::add_bot);
    cfg.service(bots::endpoints::update_bot);
    cfg.service(bots::endpoints::delete_bot);
    cfg.service(listeners::endpoints::get_listeners);
    cfg.service(listeners::endpoints::add_listener);
}

/// Helper function to send HTTP requests
pub fn send_request<T: serde::Serialize>(
    client: &Client,
    method: Method,
    url: &str,
    body: Option<T>,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let request = match method {
        Method::POST => client.post(url),
        Method::PUT => client.put(url),
        Method::DELETE => client.delete(url),
        Method::GET => client.get(url),
        _ => panic!("Unsupported HTTP method"),
    };

    if let Some(body) = body {
        request.json(&body).send()
    } else {
        request.send()
    }
}
