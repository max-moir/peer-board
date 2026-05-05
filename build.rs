use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir(std::env::var("OUT_DIR").unwrap()) 
        .compile_protos(
            &[
                "proto/peerboard.proto",          
                "proto/challenge.proto",          
            ],
            &["proto"], 
        )
        .unwrap();

    Ok(())
}