//! Grammars that share an external scanner upstream each carry a copy of it, so every
//! grammar crate builds on its own. The copies have to stay identical to the originals.

use std::path::Path;

const COPIES: [(&str, &str); 3] = [
    ("typescript", "tsx"),
    ("php", "php_only"),
    ("ocaml", "ocaml_interface"),
];

#[test]
fn scanner_copies_match_their_originals() {
    let grammars = Path::new(env!("CARGO_MANIFEST_DIR")).join("../grammars");
    for (original, copy) in COPIES {
        let read = |key: &str| {
            let path = grammars.join(key).join("src/scanner.rs");
            std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()))
        };
        assert!(
            read(original) == read(copy),
            "grammars/{copy}/src/scanner.rs differs from grammars/{original}/src/scanner.rs"
        );
    }
}
