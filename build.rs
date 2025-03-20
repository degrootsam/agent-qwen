extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=tree-sitter-python/src/parser.c");
    println!("cargo:rerun-if-changed=tree-sitter-python/src/scanner.c");

    cc::Build::new()
        .include("tree-sitter-python/src")
        .file("tree-sitter-python/src/parser.c")
        .file("tree-sitter-python/src/scanner.c")
        .compile("tree_sitter_python");
}
