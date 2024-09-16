#![allow(dead_code)]

use crate::configs::Configs;
use crate::prelude::*;

lazy_static! {
    pub static ref CONFIGS: Configs = {
        let raw = std::fs::read_to_string("configs.toml").unwrap();
        toml::from_str(&raw).unwrap()
    };
}
