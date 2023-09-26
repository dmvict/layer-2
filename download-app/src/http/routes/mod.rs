use actix_web::web::ServiceConfig;

mod index;

pub fn configure(cfg: &mut ServiceConfig) {
  cfg.configure(index::configure);
}
