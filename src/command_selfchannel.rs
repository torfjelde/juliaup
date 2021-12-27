#[cfg(not(feature = "windowsstore"))]
use crate::config_file::*;
#[cfg(not(feature = "windowsstore"))]
use anyhow::{bail, Context};
use anyhow::Result;

#[cfg(not(feature = "windowsstore"))]
pub fn run_command_selfchannel(channel: String) -> Result<()> {
    let mut config_file = load_mut_config_db()
        .with_context(|| "`self update` command failed to load configuration data.")?;

    if channel != "dev" && channel != "releasepreview" && channel != "release" {
        bail!("'{}' is not a valid juliaup channel, you can only specify 'release', 'releasepreview' or 'dev'.", channel);
    }

    config_file.data.juliaup_channel = Some(channel.clone());

    save_config_db(config_file)
        .with_context(|| "`selfchannel` command failed to save configuration db.")?;

    Ok(())
}

#[cfg(feature = "windowsstore")]
pub fn run_command_selfchannel(_channel: String) -> Result<()> {
    println!("This command is currently not supported in the Windows Store distributed version of juliaup.");

    Ok(())
}
