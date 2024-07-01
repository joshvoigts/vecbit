use crate::db::Pool;
use crate::p;
use actix_web::cookie::Key;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

pub struct AppConfig {
   pub cookie_ttl: i64,
   pub bind_address: String,
   pub bind_port: u16,
   pub db_path: String,
   pub environment: String,
   pub session_master_key: Key,
   pub smtp_email: String,
   pub smtp_password: String,
   pub static_path: String,
}

impl AppConfig {
   pub fn new_from_env() -> Self {
      let bind_address = std::env::var("VECB_BIND_ADDRESS")
         .expect("VECB_BIND_ADDRESS must be set");
      let bind_port: u16 = std::env::var("VECB_BIND_PORT")
         .expect("VECB_BIND_PORT must be set")
         .parse()
         .expect("VECB_BIND_PORT must be a u16");
      let db_path = std::env::var("VECB_DB_PATH")
         .expect("VECB_DB_PATH must be set");
      let environment =
         std::env::var("VECB_ENV").expect("VECB_ENV must be set");
      let session_master_key = Key::from(
         BASE64_STANDARD
            .decode(
               std::env::var("VECB_SESSION_MASTER_KEY")
                  .expect("SESSION_MASTER_KEY must be set"),
            )
            .expect("SESSION_MASTER_KEY invalid")
            .as_slice(),
      );
      let smtp_email = std::env::var("VECB_SMTP_EMAIL")
         .expect("VECB_SMTP_EMAIL must be set");
      let smtp_password = std::env::var("VECB_SMTP_PASSWORD")
         .expect("VECB_SMTP_PASSWORD must be set");
      let static_path = std::env::var("VECB_STATIC_PATH")
         .expect("VECB_STATIC_PATH must be set");

      AppConfig {
         cookie_ttl: 60 * 60 * 24 * 30,
         bind_address: bind_address,
         bind_port: bind_port,
         db_path: db_path,
         environment: environment,
         session_master_key: session_master_key,
         smtp_email: smtp_email,
         smtp_password: smtp_password,
         static_path: static_path,
      }
   }
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
      let file = fs::File::create(static_dir.join(tmpl)).unwrap();
      let res = tera.render_to(tmpl, &context, file);
      if res.is_err() {
         println!("failed to render: {}", tmpl);
      }
   }
}
