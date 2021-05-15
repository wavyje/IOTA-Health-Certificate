use actix_web::web;
mod home;
mod content_loader;
mod profile;
mod scan;
mod success;
use super::path::Path;

pub fn app_factory(app: &mut web::ServiceConfig) {
 let base_path: Path = Path{prefix: String::from("/")};

 app.route(&base_path.define(String::from("")),
 web::get().to(home::home))

 .route(&base_path.define(String::from("profile")),
  web::get().to(profile::profile))

 .route(&base_path.define(String::from("scan")),
  web::get().to(scan::scan))

  .route(&base_path.define(String::from("success")),
  web::get().to(success::success));
}