extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/vector_tile.proto"], &["src/"]).unwrap();
}
