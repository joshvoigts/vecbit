use crate::app::AppData;
use crate::auth;
use crate::auth::{generate_token, send_magic_email};
use crate::db;
use crate::error::UserError;
use crate::model::MagicLink;
use crate::model::Start;
use crate::model::Token;
use crate::model::User;
use crate::responder::Html;
use crate::vecbit;
use actix_session::Session;
use actix_web::dev::ConnectionInfo;
use actix_web::{web, Responder};
use tera::Context;
use time::{Duration, OffsetDateTime};

const SESSION_TTL: Duration = Duration::new(60 * 60 * 24 * 30 * 3, 0);

pub async fn post_start(
   data: web::Data<AppData>,
   start: web::Form<Start>,
   info: ConnectionInfo,
   session: Session,
) -> Result<impl Responder, UserError> {
   let auth_token = generate_token(64);
   session.insert("token", &auth_token)?;
   session.insert("email", &start.email.to_lowercase())?;

   let (secret_key, encrypted_token) = auth::encrypt(&auth_token)?;

   session.insert("secret_key", &secret_key)?;

   send_magic_email(
      &start.email,
      &encrypted_token,
      Some(info),
      data.get_ref(),
   )
   .await?;

   Ok(web::Redirect::to("/start").see_other())
}

pub async fn get_login(
   data: web::Data<AppData>,
   params: web::Query<MagicLink>,
   session: Session,
) -> Result<impl Responder, UserError> {
   let sess_token = session
      .get::<String>("token")?
      .ok_or(UserError::NotAuthorized)?;
   let sess_secret_key = session
      .get::<String>("secret_key")?
      .ok_or(UserError::NotAuthorized)?;
   let sess_email = session
      .get::<String>("email")?
      .ok_or(UserError::NotAuthorized)?;

   let email_token = auth::decrypt(sess_secret_key, &params.token)?;

   if sess_token != email_token {
      return Err(UserError::NotAuthorized);
   }

   let conn = db::get_conn(&data.pool)?;

   let res = db::get_user_by_email(&conn, &sess_email);
   let db_user = match res {
      Err(UserError::NotFound) => {
         let user = User {
            id: 0,
            email: sess_email.clone(),
            bools: 0u64,
         };
         db::add_user(&conn, &user)
      }
      o => o,
   }?;

   let token = Token {
      id: sess_token,
      expiry: OffsetDateTime::now_utc() + SESSION_TTL,
      user_id: db_user.id,
   };
   db::add_token(&conn, &token)?;

   Ok(web::Redirect::to("/user"))
}

pub async fn get_start(
   data: web::Data<AppData>,
) -> Result<impl Responder, UserError> {
   let context = Context::new();
   Ok(Html(data.tera.render("site/start.html", &context)?))
}

pub async fn get_signup(
   data: web::Data<AppData>,
) -> Result<impl Responder, UserError> {
   let context = Context::new();
   Ok(Html(data.tera.render("site/signup.html", &context)?))
}

pub async fn get_index(
   data: web::Data<AppData>,
   session: Session,
) -> Result<impl Responder, UserError> {
   let conn = db::get_conn(&data.pool)?;
   match auth::verify(&session, &conn) {
      Ok(_) => {
         let mut context = Context::new();
         let is_valid_user = true;
         context.insert("is_valid_user", &is_valid_user);
         Ok(Html(data.tera.render("site/index.html", &context)?))
      }
      Err(UserError::NotAuthorized) => {
         let mut context = Context::new();
         let is_valid_user = false;
         context.insert("is_valid_user", &is_valid_user);
         Ok(Html(data.tera.render("site/index.html", &context)?))
      }
      Err(err) => Err(err),
   }
}

pub async fn get_user(
   data: web::Data<AppData>,
   session: Session,
) -> Result<impl Responder, UserError> {
   let conn = db::get_conn(&data.pool)?;
   let user_id = auth::verify(&session, &conn)?;
   let db_user = db::get_user(&conn, user_id)?;

   let mut context = Context::new();
   let bools = vecbit::get_bits(&db_user.bools);
   context.insert("bools", &bools);
   Ok(Html(data.tera.render("site/user.html", &context)?))
}

// fn sget(session: &Session, key: &str) -> Result<String, UserError> {
//    session.get(key)?.ok_or(UserError::InternalError(
//       "could not get cookie token".to_string(),
//    ))
// }
