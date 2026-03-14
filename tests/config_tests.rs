// tests\config_tests.rs
use rs_foundry::config::paths::DataPaths;

#[test]
fn data_paths_are_built_from_root() {
    let paths = DataPaths::new("./data");

    assert_eq!(paths.raw().to_string_lossy(), "./data/raw");
    assert_eq!(paths.bronze().to_string_lossy(), "./data/bronze");
    assert_eq!(paths.silver().to_string_lossy(), "./data/silver");
    assert_eq!(paths.gold().to_string_lossy(), "./data/gold");
    assert_eq!(paths.checkpoints().to_string_lossy(), "./data/checkpoints");
    assert_eq!(paths.runs().to_string_lossy(), "./data/runs");
}
