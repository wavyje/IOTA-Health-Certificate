use actix_web::web;
mod home;
use super::path::Path;
mod content_loader;

pub fn app_factory(app: &mut web::ServiceConfig) {
 let base_path: Path = Path{prefix: String::from("/")};
 app.route(&base_path.define(String::from("")),
 web::get().to(home::home));
}