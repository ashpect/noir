#![forbid(unsafe_code)]
#![cfg_attr(not(test), warn(unused_crate_dependencies, unused_extern_crates))]

//! Nargo is the package manager for Noir
//! This name was used because it sounds like `cargo` and
//! Noir Package Manager abbreviated is npm, which is already taken.

pub mod cli;
mod errors;

use std::env;

use color_eyre::config::HookBuilder;

use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

// TODO: Currently only used by benches.
use noir_artifact_cli as _;
// NOTE: Currently only used for fuzzing.
use nargo_cli as _;

const PANIC_MESSAGE: &str = "This is a bug. We may have already fixed this in newer versions of Nargo so try searching for similar issues at https://github.com/noir-lang/noir/issues/.\nIf there isn't an open issue for this bug, consider opening one at https://github.com/noir-lang/noir/issues/new?labels=bug&template=bug_report.yml";

fn main() {
    setup_tracing();

    // Register a panic hook to display more readable panic messages to end-users
    let (panic_hook, _) =
        HookBuilder::default().display_env_section(false).panic_section(PANIC_MESSAGE).into_hooks();
    panic_hook.install();

    if let Err(report) = cli::start_cli() {
        eprintln!("{report:#}");
        std::process::exit(1);
    }
}

fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_env("NOIR_LOG"));

    if let Ok(log_dir) = env::var("NARGO_LOG_DIR") {
        let debug_file = rolling::daily(log_dir, "nargo-log");
        subscriber.with_writer(debug_file).with_ansi(false).json().init();
    } else {
        subscriber.with_writer(std::io::stderr).with_ansi(true).init();
    }
}
