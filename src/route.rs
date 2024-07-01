use crate::controller::api;
use crate::controller::ui;
use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
   cfg.service(
      web::resource("/").route(web::get().to(ui::get_user)),
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
   .service(web::resource("/bools/{index}").route(web::put().to(ui::put_bool)))
   .service(
      web::scope("/api/v1")
         .service(
            web::resource("/bools")
               .route(web::get().to(api::get_bools)),
         )
         .service(
            web::resource("/bools/{index}")
               .route(web::get().to(api::get_bool))
               .route(web::put().to(api::put_bool)),
         ),
   );
}
