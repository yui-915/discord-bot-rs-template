#![allow(unused_imports)]
#![allow(dead_code)]

use crate::data::Data;

pub use serde::{Deserialize, Serialize};

pub use poise::{command, serenity_prelude::*, PrefixFrameworkOptions};

pub use crate::statics::*;

pub use lazy_static::lazy_static;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type CommandResult = Result<(), Error>;
pub type FrameworkOptions = poise::FrameworkOptions<Data, Error>;
pub type Command = poise::Command<
    <Context<'static> as poise::_GetGenerics>::U,
    <Context<'static> as poise::_GetGenerics>::E,
>;
