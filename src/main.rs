// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::config::AppConfig;
use actix_files::Files;
use actix_session::{
   config::PersistentSession, storage::CookieSessionStore,
   SessionMiddleware,
};
use actix_web::cookie::time::Duration;
use actix_web::{middleware, web, App, HttpServer};
use db::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::env;
use std::io;
use tera::Tera;

mod api;
mod app;
mod auth;
mod config;
mod db;
mod error;
mod init;
mod model;
mod p;
mod responder;
mod ui;
mod vecbit;

#[actix_web::main]
async fn main() -> io::Result<()> {
   dotenvy::dotenv().ok();

   let tera = Tera::new("web/templates/**/*.html")
      .expect("Failed to render templates");
   // app::build_static(&tera);

   let args: Vec<String> = env::args().skip(1).collect();
   for (i, arg) in args.iter().enumerate() {
      match arg.as_str() {
         "init_db" => init::init_db(),
         "add_user" => init::add_user(&args[i + 1..args.len()]),
         "gen_session_master" => init::gen_session_master(),
         _ => init::fail("Unknown argument"),
      }
   }
   if args.len() > 0 {
      return Ok(());
   }

   let config = AppConfig::new_from_env();

   std::env::set_var("RUST_LOG", "debug");
   std::env::set_var("RUST_BACKTRACE", "1");
   env_logger::init_from_env(
      env_logger::Env::new().default_filter_or("info"),
   );
   log::info!(
      "starting server at http://{}:{}",
      config.bind_address,
      config.bind_port
   );

   let manager = SqliteConnectionManager::file(config.db_path);
   let pool = Pool::new(manager).unwrap();

   let app_data = app::AppData {
      pool: pool,
      tera: tera,
      smtp_email: config.smtp_email,
      smtp_password: config.smtp_password,
   };

   HttpServer::new(move || {
      App::new()
         .app_data(web::Data::new(app_data.clone()))
         .wrap(middleware::Logger::default())
         .wrap(
            SessionMiddleware::builder(
               CookieSessionStore::default(),
               config.session_master_key.clone(),
            )
            .session_lifecycle(
               PersistentSession::default()
                  .session_ttl(Duration::seconds(config.cookie_ttl)),
            )
            .cookie_secure(config.environment != "development")
            .build(),
         )
         .configure(app::service_config)
         .service(
            Files::new("/", config.static_path.clone())
               // .index_file("site/index.html")
               // .default_handler(fn_service(
               //    |req: ServiceRequest| async {
               //       let (req, _) = req.into_parts();
               //       let file = NamedFile::open_async("./static/site/404.html")
               //          .await?;
               //       let res = file.into_response(&req);
               //       Ok(ServiceResponse::new(req, res))
               //    },
               // ))
               .prefer_utf8(true),
         )
   })
   .bind((config.bind_address, config.bind_port))?
   .workers(1)
   .run()
   .await
}
