use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Student {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub grade: u8,
}
