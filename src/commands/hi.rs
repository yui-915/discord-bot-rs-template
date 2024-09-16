use super::*;

#[command(slash_command, prefix_command)]
pub async fn hi(ctx: Context<'_>) -> CommandResult {
    ctx.say("Hello!").await?;
    Ok(())
}
