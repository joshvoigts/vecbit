use crate::model::User;
use actix_web::cookie::Key;
use base64::{
   display::Base64Display, engine::general_purpose::STANDARD,
};
use rusqlite::Connection;
use std::process;
use std::str;

pub fn add_user(args: &[String]) {
   let conn = get_conn();
   let user = User {
      id: 0,
      email: args[0].clone(),
      bools: 0u64,
   };
   conn
      .execute("INSERT INTO user (email) VALUES (?1)", (&user.email,))
      .expect("failed to add new user");
}

pub fn gen_session_master() {
   let secret_key = Key::generate();
   let value = Base64Display::new(secret_key.master(), &STANDARD);
   println!("Session secret:");
   println!("{}", value);
}

pub fn init_db() {
   let conn = get_conn();
   conn
      .execute_batch(
         "BEGIN;
         CREATE TABLE user (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            bools INTEGER NOT NULL DEFAULT 0
         );

         CREATE TABLE token (
            id TEXT PRIMARY KEY,
            expiry TEXT NOT NULL,
            user_id INTEGER NOT NULL REFERENCES user
         );
         COMMIT;",
      )
      .expect("failed to create user table");
}

fn get_conn() -> Connection {
   Connection::open("db.sqlite").expect("db connection error")
}

pub fn fail(message: &str) -> ! {
   println!("{}", String::from(message));
   process::exit(1)
}
