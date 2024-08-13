use crate::services::auth_service::{
    login_user, logout, profile_user, refresh_token, register_user,
};
use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/register").route(web::post().to(register_user)))
            .service(web::resource("/login").route(web::post().to(login_user)))
            .service(web::resource("/profile").route(web::get().to(profile_user)))
            .service(web::resource("/refresh_token").route(web::post().to(refresh_token)))
            .service(web::resource("/logout").route(web::post().to(logout))),
    );
}
