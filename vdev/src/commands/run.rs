use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Args;

use crate::{app, features};

/// Run `vector` with the minimum set of features required by the config file
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// Build and run `vector` in debug mode (default)
    #[arg(long, default_value_t = true)]
    debug: bool,

    /// Build and run `vector` in release mode
    #[arg(long)]
    release: bool,

    /// Path to configuration file
    config: PathBuf,

    /// Non-config arguments to `vector`
    args: Vec<String>,
}

impl Cli {
    pub(super) fn exec(self) -> Result<()> {
        if self.debug && self.release {
            bail!("Can only set one of `--debug` and `--release`");
        }

        let features = features::load_and_extract(&self.config)?;
        let mut args = vec!["run", "--no-default-features", "--features", &features];
        if self.release {
            args.push("--release");
        }
        args.extend([
            "--",
            "--config",
            self.config.to_str().expect("Invalid config file name"),
        ]);
        args.extend(self.args.iter().map(String::as_str));
        app::exec("cargo", args)
    }
}
