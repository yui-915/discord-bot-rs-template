#[path = "../configs.rs"]
mod configs;
#[path = "../data.rs"]
mod data;
#[path = "../options.rs"]
mod options;
mod prelude;
mod statics;

use crate::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    ClientBuilder::new(options::token(), options::intents())
        .framework(
            poise::Framework::builder()
                .options(FrameworkOptions {
                    commands: internal_macros::commands!(),
                    ..options::options()
                })
                .setup(move |ctx, ready, framework| {
                    Box::pin(async move { Ok(data::data(ctx, ready, framework)) })
                })
                .build(),
        )
        .await
        .unwrap()
        .start()
        .await
        .unwrap();
}
