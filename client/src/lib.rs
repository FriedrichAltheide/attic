#![deny(
    asm_sub_register,
    deprecated,
    missing_abi,
    unsafe_code,
    unused_macros,
    unused_must_use,
    unused_unsafe
)]
#![deny(clippy::from_over_into, clippy::needless_question_mark)]
#![cfg_attr(
    not(debug_assertions),
    deny(unused_imports, unused_mut, unused_variables,)
)]

pub mod api;
pub mod cache;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "cli")]
mod command;
pub mod config;
#[cfg(feature = "cli")]
mod nix_config;
#[cfg(feature = "cli")]
mod nix_netrc;
#[cfg(feature = "nix_store")]
mod push;
mod version;
