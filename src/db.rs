use crate::error::UserError;
use crate::model::AccessToken;
use crate::model::Token;
use crate::model::User;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection =
   r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn get_conn(pool: &Pool) -> Result<Connection, UserError> {
   Ok(pool.get()?)
}

pub fn add_user(
   conn: &Connection,
   user: &User,
) -> Result<User, UserError> {
   let mut stmt = conn.prepare_cached(
      "INSERT INTO user ('email', 'bools') VALUES (?1, ?2)
      RETURNING *",
   )?;
   let user = stmt.query_row((&user.email, &user.bools), |row| {
      Ok(User {
         id: row.get("id")?,
         email: row.get("email")?,
         bools: row.get("bools")?,
      })
   })?;
   Ok(user)
}

pub fn get_user(
   conn: &Connection,
   user_id: usize,
) -> Result<User, UserError> {
   let mut stmt =
      conn.prepare_cached("SELECT * FROM user WHERE user.id = ?1")?;
   let user = stmt.query_row((user_id,), |row| {
      Ok(User {
         id: row.get("id")?,
         email: row.get("email")?,
         bools: row.get("bools")?,
      })
   })?;
   Ok(user)
}

pub fn get_user_by_email(
   conn: &Connection,
   email: &String,
) -> Result<User, UserError> {
   let mut stmt = conn
      .prepare_cached("SELECT * FROM user WHERE user.email = ?1")?;
   let user = stmt.query_row((email,), |row| {
      Ok(User {
         id: row.get("id")?,
         email: row.get("email")?,
         bools: row.get("bools")?,
      })
   })?;
   Ok(user)
}

pub fn set_user(
   conn: &Connection,
   user: User,
) -> Result<User, UserError> {
   let mut stmt = conn.prepare_cached(
      "UPDATE user SET email = ?2, bools = ?3
         WHERE user.id = ?1 RETURNING *",
   )?;
   let user =
      stmt.query_row((user.id, user.email, user.bools), |row| {
         Ok(User {
            id: row.get("id")?,
            email: row.get("email")?,
            bools: row.get("bools")?,
         })
      })?;
   Ok(user)
}

pub fn add_token(
   conn: &Connection,
   token: &Token,
) -> Result<Token, UserError> {
   let mut stmt = conn.prepare_cached(
      "INSERT INTO token ('id', 'user_id')
      VALUES (?1, ?2)
      RETURNING *",
   )?;
   let token =
      stmt.query_row((&token.id, &token.user_id), |row| {
         Ok(Token {
            id: row.get("id")?,
            user_id: row.get("user_id")?,
         })
      })?;
   Ok(token)
}

pub fn get_token(
   conn: &Connection,
   id: &String,
) -> Result<Token, UserError> {
   let mut stmt = conn
      .prepare_cached("SELECT * FROM token WHERE token.id = ?1")?;
   let token = stmt.query_row((id,), |row| {
      Ok(Token {
         id: row.get("id")?,
         user_id: row.get("user_id")?,
      })
   })?;
   Ok(token)
}

pub fn get_access_tokens(
   conn: &Connection,
   user_id: &String,
) -> Result<Vec<AccessToken>, UserError> {
   let mut stmt = conn.prepare_cached(
      "SELECT * FROM access_token WHERE access_token.user_id = ?1",
   )?;
   let mut tokens = Vec::new();
   let mut rows = stmt.query((user_id,))?;
   while let Some(row) = rows.next()? {
      tokens.push(AccessToken {
         id: row.get("id")?,
         user_id: row.get("user_id")?,
      });
   };
   Ok(tokens)
}
