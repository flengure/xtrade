pub mod bots;
pub mod listeners;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(bots::get_bots);
    cfg.service(bots::get_bot_by_id);
    cfg.service(bots::add_bot);
    cfg.service(bots::update_bot);
    cfg.service(bots::delete_bot);
    cfg.service(listeners::get_listeners);
    cfg.service(listeners::add_listener);
}
