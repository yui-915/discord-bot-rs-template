use crate::prelude::*;
use poise::serenity_prelude::Context;

pub struct Data {}
pub fn data(_ctx: &Context, ready: &Ready, _framework: &dyn Framework) -> Data {
    println!("Ready as {}", ready.user.name);
    Data {}
}
