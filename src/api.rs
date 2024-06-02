use crate::app::AppData;
use crate::db;
use crate::error::UserError;
use crate::vecbit;
use actix_web::{web, HttpResponse};
use serde_json::Value;

// async fn verify_user(user: User) -> Result<(), Error> {
//    let password_hash = user.token_hashed;
//    let parsed_hash = PasswordHash::new(&password_hash)?;
//    Argon2::default()
//       .verify_password(token_hashed, &parsed_hash)
//       .map_err(ErrorForbidden)?;
//    Ok()
// }

// pub async fn get_user(
//    data: web::Data<AppData>,
//    path: web::Path<usize>,
// ) -> Result<HttpResponse, UserError> {
//    let user_id = path.into_inner();
//    let conn = db::get_conn(&data.pool)?;
//    let db_user = db::get_user(&conn, user_id)?;
//    //    verify_user(db_user)?;
//    Ok(HttpResponse::Ok().json(db_user))
// }

pub async fn get_bool(
   data: web::Data<AppData>,
   path: web::Path<(usize, usize)>,
) -> Result<HttpResponse, UserError> {
   let (user_id, index) = path.into_inner();
   if index >= 63 {
      return Err(UserError::IndexOutOfRange);
   }
   let conn = db::get_conn(&data.pool)?;
   let db_user = db::get_user(&conn, user_id)?;
   //    verify_user(db_user)?;
   let bit = vecbit::get_bit(&db_user.bools, index)?;
   Ok(HttpResponse::Ok().json(bit))
}

pub async fn get_bools(
   data: web::Data<AppData>,
   path: web::Path<usize>,
) -> Result<HttpResponse, UserError> {
   let user_id = path.into_inner();
   let conn = db::get_conn(&data.pool)?;
   let db_user = db::get_user(&conn, user_id)?;
   //    verify_user(db_user)?;
   let bools = vecbit::get_bits(&db_user.bools);
   Ok(HttpResponse::Ok().json(bools))
}

const BAD_BOOL_ERROR: &str = "expected true, false, 1, or 0";

pub async fn put_bool(
   data: web::Data<AppData>,
   path: web::Path<(usize, usize)>,
   json: web::Json<Value>,
) -> Result<HttpResponse, UserError> {
   let json = match json.into_inner() {
      Value::Bool(val) => val,
      Value::Number(val) => {
         let val = val.as_u64().ok_or(UserError::BadRequest(
            BAD_BOOL_ERROR.to_string(),
         ))?;
         val != 0
      }
      _ => {
         return Err(UserError::BadRequest(
            BAD_BOOL_ERROR.to_string(),
         ))
      }
   };
   let (user_id, index) = path.into_inner();
   if index >= 63 {
      return Err(UserError::IndexOutOfRange);
   }
   let conn = db::get_conn(&data.pool)?;
   let mut db_user = db::get_user(&conn, user_id)?;
   //       verify_user(db_user)?;
   vecbit::set_bit(&mut db_user.bools, index, json);
   db::set_user(&conn, db_user)?;
   Ok(HttpResponse::Ok().into())
}
