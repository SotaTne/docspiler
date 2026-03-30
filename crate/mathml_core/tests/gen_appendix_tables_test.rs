#[path = "../src/gen/mod.rs"]
mod gen_support;

use std::fs;
use std::path::PathBuf;

fn snapshot_html() -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(
        manifest_dir
            .join("assets")
            .join("CR-mathml-core-20250624.html"),
    )
    .expect("read snapshot html")
}

#[test]
fn parses_expected_appendix_table_counts_from_snapshot() {
    let html = snapshot_html();
    let tables = gen_support::parse_appendix_tables(&html);

    assert_eq!(tables.b2.len(), 1177);
    assert_eq!(tables.b3_combining.len(), 39);
    assert_eq!(tables.b3_non_combining.len(), 41);
    assert_eq!(tables.b4.len(), 41);
    assert_eq!(tables.c1.len(), 112);
}

#[test]
fn renders_generated_rust_from_snapshot() {
    let html = snapshot_html();
    let tables = gen_support::parse_appendix_tables(&html);
    let generated = gen_support::generate_appendix_tables_rs(&tables);

    assert!(generated.contains("FULL_OPERATOR_DICTIONARY_ENTRIES"));
    assert!(generated.contains("FULL_COMBINING_CHARACTER_ENTRIES"));
    assert!(generated.contains("FULL_GLYPH_ASSEMBLY_ENTRIES"));
    assert!(generated.contains("FULL_ITALIC_MAPPING_ENTRIES"));
    assert!(generated.contains("\\u{2211}"));
    assert!(tables.b2.iter().any(|entry| entry.content == "∑"));
}
