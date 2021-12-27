use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub capital: String,
    pub area: usize,
}
