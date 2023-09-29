use crate::Context;
use crate::Error;
use poise::builtins;
use poise::builtins::HelpConfiguration;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "このbotのヘルプを表示します"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    builtins::help(
        ctx,
        command.as_deref(),
        HelpConfiguration {
            extra_text_at_bottom: "TestTest",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
