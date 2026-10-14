//! Reads the sample project files shipped with the original sconcho.

use charto_core::spf::{parse_project, serialize_project, RowLabelMode};
use charto_core::symbols::SymbolLibrary;
use std::path::PathBuf;

#[test]
fn reads_api1_sample() {
    let p = parse_project(include_bytes!("data/test_api_1.spf")).expect("api 1");
    assert_eq!(p.api_version, 1);
    assert_eq!(p.grid_items.len(), 224);
    assert_eq!(p.legend_items.len(), 5);
    assert_eq!(p.colors.len(), 9);
    assert_eq!(p.pattern_repeats.len(), 1);
    assert_eq!(p.settings.label_font.family, "Myriad Pro");
    assert_eq!(p.settings.legend_font.point_size, 18.0);
    assert_eq!(p.settings.grid_cell_width, 36);
    assert_eq!(p.settings.grid_cell_height, 30);
    assert_eq!(p.settings.row_label_mode, RowLabelMode::AllRows);
    assert!(p.active_symbol.is_none());
}

#[test]
fn reads_api2_sample_with_missing_extended_settings() {
    // this file is labelled API 2 but ends right after the cell size
    let p = parse_project(include_bytes!("data/test_api_2.spf")).expect("api 2");
    assert_eq!(p.api_version, 2);
    assert_eq!(p.grid_items.len(), 224);
    assert_eq!(p.settings.row_label_mode, RowLabelMode::ShowOddRows);
    assert_eq!(p.settings.row_label_start, 1); // default kept
}

#[test]
fn reads_insert_delete_sample() {
    let p = parse_project(include_bytes!("data/test_delete_insert_rows_columns.spf"))
        .expect("insert/delete sample");
    assert!(!p.grid_items.is_empty());
}

#[test]
fn legacy_files_resave_as_api3_and_reload_identically() {
    let p = parse_project(include_bytes!("data/test_api_1.spf")).unwrap();
    let bytes = serialize_project(&p);
    let mut q = parse_project(&bytes).unwrap();
    assert_eq!(q.api_version, 3);
    q.api_version = p.api_version;
    assert_eq!(p, q);
}

#[test]
fn bundled_symbol_library_loads_completely() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/symbols");
    let (lib, skipped) = SymbolLibrary::load(&[root]);
    assert!(skipped.is_empty(), "unreadable symbols: {skipped:?}");
    assert!(lib.len() >= 80);
    let knit = lib.get("knit").expect("knit symbol");
    assert_eq!(knit.width, 1);
    assert!(knit.svg_path.exists());
    // every symbol used by the sample project exists in the library
    let p = parse_project(include_bytes!("data/test_api_1.spf")).unwrap();
    for it in &p.grid_items {
        assert!(lib.get(&it.name).is_some(), "missing symbol {}", it.name);
    }
}

#[test]
fn palette_groups_by_category_name() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/symbols");
    let (lib, _) = SymbolLibrary::load(&[root]);
    let cats = lib.categories();
    let names: Vec<&str> = cats.iter().map(|(c, _)| c.as_str()).collect();
    // alphabetical, and each category name appears exactly once
    let mut sorted = names.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(names, sorted);
    let total: usize = cats.iter().map(|(_, s)| s.len()).sum();
    assert_eq!(total, lib.len());
}
