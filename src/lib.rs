pub mod networking;
pub mod simulation;
pub mod util;
mod server;

use failure::Error;
use failure::format_err;

use eternalreckoning_core::util::config::Config;
use eternalreckoning_core::util::logging;

pub struct Bootstrap {
    pub args: Vec<String>,
    pub config: Option<String>,
}

pub fn run(bootstrap: Bootstrap) -> Result<(), Error> {
    let config = initialize(bootstrap)?;

    server::main(config)?;

    Ok(())
}

fn initialize(bootstrap: Bootstrap)
    -> Result<util::config::Config, Error>
{
    let config = get_configuration(bootstrap)?;
    let config = config.data;

    logging::configure(&config.logging, "eternalreckoning_server")?;

    Ok(config)
}

fn get_configuration(bootstrap: Bootstrap)
    -> Result<Config<util::config::Config>, Error>
{
    match bootstrap.config {
        Some(path) => {
            Config::<util::config::Config>::from_file(&path)
                .or_else(|_| {
                    Config::<util::config::Config>::write_default(&path)
                })
                .map_err(|e| { e.into() })
        },
        None => Err(format_err!("no configuration file path provided")),
    }
}