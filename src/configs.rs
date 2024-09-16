use crate::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct Configs {
    pub token: String,
    pub prefix: String,
}
