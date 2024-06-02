use crate::api;
use crate::db::Pool;
use crate::p;
use crate::ui;
use actix_web::web;
use std::fs;
use std::fs::File;
use std::path::Path;
use tera::{Context, Tera};

pub fn service_config(cfg: &mut web::ServiceConfig) {
   cfg.service(
      web::resource("/").route(web::get().to(ui::get_index)),
   )
   .service(
      web::resource("/start")
         .route(web::post().to(ui::post_start))
         .route(web::get().to(ui::get_start)),
   )
   .service(
      web::resource("/signup").route(web::get().to(ui::get_signup)),
   )
   .service(
      web::resource("/login").route(web::get().to(ui::get_login)),
   )
   .service(web::resource("/user").route(web::get().to(ui::get_user)))
   .service(
      web::scope("/api/v1")
         .service(
            web::resource("/bool")
               .route(web::get().to(api::get_bools)),
         )
         .service(
            web::resource("/bool/{index}")
               .route(web::get().to(api::get_bool))
               .route(web::put().to(api::put_bool)),
         ),
   );
}

#[derive(Clone)]
pub struct AppData {
   pub pool: Pool,
   pub tera: Tera,
   pub smtp_email: String,
   pub smtp_password: String,
}

pub fn build_static(tera: &Tera) {
   p!("Building static pages");
   let static_dir = Path::new("./web/static");
   let site_dir = static_dir.join("site");
   if site_dir.exists() {
      fs::remove_dir_all(site_dir).unwrap();
   }
   for tmpl in tera.get_template_names() {
      let path = Path::new(tmpl);
      if !path.starts_with("site") {
         continue;
      }
      let dir = static_dir.join(path.parent().unwrap());
      fs::create_dir_all(dir).unwrap();
      let context = Context::new();
      let file = File::create(static_dir.join(tmpl)).unwrap();
      let res = tera.render_to(tmpl, &context, file);
      if res.is_err() {
         println!("failed to render: {}", tmpl);
      }
   }
}
