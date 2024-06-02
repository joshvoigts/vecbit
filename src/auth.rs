use crate::app::AppData;
use crate::db;
use crate::error::UserError;
use actix::Arbiter;
use actix_session::Session;
use actix_web::dev::ConnectionInfo;
use actix_web::http::uri;

use aes_gcm::{
   aead::{Aead, AeadCore, KeyInit, OsRng},
   Aes256Gcm,
};
// use argon2::{
//    password_hash::{
//       PasswordHasher,
//       SaltString,
//    },
//    Argon2,
// };
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use db::Connection;
use mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use rand::{distributions::Alphanumeric, Rng};
use tera::Context;

pub fn verify(
   session: &Session,
   conn: &Connection,
) -> Result<usize, UserError> {
   let sess_token = session
      .get::<String>("token")?
      .ok_or(UserError::NotAuthorized)?;

   let db_token =
      db::get_token(&conn, &sess_token).map_err(|err| match err {
         UserError::NotFound => UserError::NotAuthorized,
         e => e,
      })?;
   Ok(db_token.user_id)
}

pub fn generate_token(length: usize) -> String {
   rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(length)
      .map(char::from)
      .collect()
}

// pub fn hash_token(
//    token: &String,
//    salt: &String,
// ) -> Result<String, UserError> {
//    let salt_str = SaltString::from_b64(salt)?;
//    let argon2 = Argon2::default();
//    let hashed = argon2.hash_password(token.as_bytes(), &salt_str);
//    Ok(hashed?.to_string())
// }

const LINK_HTML_TEMPLATE: &str =
   "<h3>Click the link to login:</h3><a href=\"{}\">{}</a>";

pub async fn send_magic_email(
   email: &str,
   token: &str,
   info: Option<ConnectionInfo>,
   data: &AppData,
) -> Result<(), UserError> {
   let info = info.ok_or(UserError::InternalError(
      "failed to unwrap request info".to_string(),
   ))?;
   let link = uri::Builder::new()
      .scheme(info.scheme())
      .authority(info.host())
      .path_and_query(format!("/login?token={token}"))
      .build()
      .unwrap()
      .to_string();

   let mut context = Context::new();
   context.insert("token_link", &link);

   let email = email.to_string();
   let html_body = data.tera.render("email/magic_link.html", &context)?;
   let smtp_email = data.smtp_email.clone();
   let smtp_password = data.smtp_password.clone();

   let message = MessageBuilder::new()
      .from("no.reply.vecbit@gmail.com")
      .to(email.clone())
      .subject("Login to vecbit")
      .html_body(html_body);
//       .text_body("Click the link to login: "); // TODO

   Arbiter::new().spawn(async move {
      // Connect to the SMTP submissions port, upgrade to TLS and
      // authenticate using the provided credentials.
      let res = SmtpClientBuilder::new("smtp.gmail.com", 587)
         .implicit_tls(false)
         .credentials((
            smtp_email.as_str(),
            smtp_password.as_str(),
         ))
         .connect()
         .await;
      match res {
         Ok(mut conn) => match conn.send(message).await {
            Ok(_) => (),
            Err(err2) => {
               log::error!(
                  "Failed to send email: {} {}",
                  email,
                  err2
               );
            }
         },
         Err(err) => {
            log::error!(
               "Failed to connect to email server: {} {}",
               email,
               err
            );
         }
      };
   });

   Ok(())
}

pub fn encrypt(
   message: &String,
) -> Result<(String, String), UserError> {
   let key = Aes256Gcm::generate_key(OsRng);
   let cipher = Aes256Gcm::new(&key);
   let iv = Aes256Gcm::generate_nonce(&mut OsRng);
   let ciphertext = cipher.encrypt(&iv, message.as_bytes())?;
   let data = URL_SAFE.encode([iv.to_vec(), ciphertext].concat());
   let secret = URL_SAFE.encode(key);
   Ok((secret, data))
}

pub fn decrypt(
   secret: String,
   data: &String,
) -> Result<String, UserError> {
   let key = URL_SAFE.decode(secret)?;
   let decoded = URL_SAFE.decode(data)?;
   let cipher = Aes256Gcm::new_from_slice(&key)?;
   let iv = aes_gcm::Nonce::from_slice(&decoded[..12]);
   let ciphertext = &decoded[12..];
   let message = cipher.decrypt(iv, ciphertext)?;
   Ok(String::from_utf8(message)?)
}
