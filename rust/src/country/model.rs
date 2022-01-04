use serde::{Deserialize, Serialize};

// define our model
#[derive(Serialize, Deserialize, Debug, Default)]
//give default value
#[serde(default)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub capital: String,
    pub area: usize,
}
