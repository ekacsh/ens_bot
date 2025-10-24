use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn off(ctx: Context<'_>) -> Result<(), Error> {
    let response =
        "This is a un-automated <#898337415939371049> reminder. Please click on the linked thingy.";
    ctx.say(response).await?;
    Ok(())
}
