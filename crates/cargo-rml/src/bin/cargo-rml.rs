use cargo_rml::options::Args;
use clap::Parser;
use std::{
    env,
    process::{exit, Command},
};

fn main() {
    let args = Args::parse_from(std::env::args().skip(1));

    let rml_rustc_path = std::env::current_exe()
        .expect("current executable path invalid")
        .with_file_name("rml-rustc");

    let cargo_path = env::var("CARGO_PATH").unwrap_or_else(|_| "cargo".to_string());
    let cargo_cmd = if std::env::var_os("RML_CONTINUE").is_some() {
        "build"
    } else {
        "check"
    };

    let mut cmd = Command::new(cargo_path);
    cmd.arg(&cargo_cmd)
        .args(args.rust_flags)
        .env("RUSTC_WRAPPER", rml_rustc_path)
        .env("CARGO_RML", "1");

    cmd.env("RML_ARGS", serde_json::to_string(&args.rml).unwrap());

    let exit_status = cmd.status().expect("could not run cargo");
    if !exit_status.success() {
        exit(exit_status.code().unwrap_or(-1));
    }
}
