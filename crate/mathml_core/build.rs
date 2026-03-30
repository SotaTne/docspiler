use std::env;
use std::fs;
use std::path::PathBuf;

#[path = "src/gen/mod.rs"]
mod gen_support;

const SNAPSHOT_HTML: &str = "assets/CR-mathml-core-20250624.html";
const GEN_MODULE: &str = "src/gen/mod.rs";

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rust_analyzer)");
    println!("cargo:rerun-if-changed={SNAPSHOT_HTML}");
    println!("cargo:rerun-if-changed={GEN_MODULE}");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let html = fs::read_to_string(manifest_dir.join(SNAPSHOT_HTML)).expect("read snapshot html");
    let tables = gen_support::parse_appendix_tables(&html);
    let generated = gen_support::generate_appendix_tables_rs(&tables);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("appendix_tables.rs"), generated)
        .expect("write generated appendix tables");
}
