use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MagicLink {
   pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Start {
   pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
   pub id: usize,
   pub email: String,
   pub bools: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
   pub id: String,
   pub user_id: usize,
}
