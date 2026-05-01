// build.rs
fn main() {
    prost_build::compile_protos(
        &["proto/peerboard.proto"],
        &["proto"],
    ).unwrap();
}