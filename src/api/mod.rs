use actix_web::web;

pub mod authentication;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("/api/authentication")
          .service(authentication::controllers::register)
          .service(authentication::controllers::login)
  );
}