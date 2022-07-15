use std::io::stdin;

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use warpgate_common::helpers::hash::hash_password;

pub(crate) async fn command() -> Result<()> {
    let mut input = String::new();

    if atty::is(atty::Stream::Stdin) {
        input = dialoguer::Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password to be hashed")
            .interact()?;
    } else {
        stdin().read_line(&mut input)?;
    }

    let hash = hash_password(&input);
    println!("{}", hash);
    Ok(())
}
