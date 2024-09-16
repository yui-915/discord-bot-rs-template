use crate::prelude::*;

pub fn options() -> FrameworkOptions {
    FrameworkOptions {
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(CONFIGS.prefix.clone()),
            case_insensitive_commands: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn token() -> String {
    CONFIGS.token.clone()
}

pub fn intents() -> GatewayIntents {
    GatewayIntents::all()
}
