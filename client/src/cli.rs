//! Global CLI Setup.

use std::env;

use anyhow::{anyhow, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use enum_as_inner::EnumAsInner;

use crate::command::cache::{self, Cache};
#[cfg(feature = "nix_store")]
use crate::command::get_closure::{self, GetClosure};
use crate::command::login::{self, Login};
#[cfg(feature = "nix_store")]
use crate::command::push::{self, Push};
use crate::command::r#use::{self, Use};
#[cfg(feature = "nix_store")]
use crate::command::watch_store::{self, WatchStore};

/// Attic binary cache client.
#[derive(Debug, Parser)]
#[clap(version)]
#[clap(propagate_version = true)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, EnumAsInner)]
pub enum Command {
    Login(Login),
    Use(Use),
    #[cfg(feature = "nix_store")]
    Push(Push),
    Cache(Cache),
    #[cfg(feature = "nix_store")]
    WatchStore(WatchStore),

    #[cfg(feature = "nix_store")]
    #[clap(hide = true)]
    GetClosure(GetClosure),
}

/// Generate shell autocompletion files.
#[derive(Debug, Parser)]
pub struct GenCompletions {
    /// The shell to generate autocompletion files for.
    shell: Shell,
}

pub async fn run() -> Result<()> {
    // https://github.com/clap-rs/clap/issues/1335
    if let Some("gen-completions") = env::args().nth(1).as_deref() {
        return gen_completions(env::args().nth(2)).await;
    }

    let opts = Opts::parse();

    match opts.command {
        Command::Login(_) => login::run(opts).await,
        Command::Use(_) => r#use::run(opts).await,
        #[cfg(feature = "nix_store")]
        Command::Push(_) => push::run(opts).await,
        Command::Cache(_) => cache::run(opts).await,
        #[cfg(feature = "nix_store")]
        Command::WatchStore(_) => watch_store::run(opts).await,
        #[cfg(feature = "nix_store")]
        Command::GetClosure(_) => get_closure::run(opts).await,
    }
}

async fn gen_completions(shell: Option<String>) -> Result<()> {
    let shell: Shell = shell
        .ok_or_else(|| anyhow!("Must specify a shell."))?
        .parse()
        .unwrap();

    clap_complete::generate(shell, &mut Opts::command(), "attic", &mut std::io::stdout());

    Ok(())
}
