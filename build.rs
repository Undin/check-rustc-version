use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::str;

fn main() {
    extract_rustc_version();
}

fn extract_rustc_version() {
    let rustc = env::var_os("RUSTC").unwrap();
    let output = Command::new(rustc).arg("--version").output().unwrap();

    let mut file = File::create("rustc-version.txt").unwrap();
    write!(&mut file, "RUSTC: {}\n", env::var_os("RUSTC").unwrap().to_string_lossy());
    let version = str::from_utf8(&output.stdout).unwrap();
    write!(&mut file, "rustc --version: {}\n", version);
}
