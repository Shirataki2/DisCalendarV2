mod id;
mod joined;
mod user;
mod config;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(joined::get_joined_guild);
    cfg.service(id::check_joined);
    cfg.service(id::get_guild);
    cfg.service(user::get_member);
    cfg.service(config::set_config);
    cfg.service(config::get_config);
}
