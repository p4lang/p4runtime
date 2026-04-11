use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define constants.
    let protoc_version = "33.0"; // Version of the Protobuf compiler to use.
    let proto_root = "../../proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("environment variable OUT_DIR not set"));
    let descriptor_path = PathBuf::from(out_dir).join("file_descriptor_set.bin");

    // Set `protoc` binary (the Protobuf compiler).
    let (protoc_bin, protoc_include) = protoc_prebuilt::init(protoc_version).unwrap();
    std::env::set_var("PROTOC", protoc_bin);
    std::env::set_var("PROTOC_INCLUDE", protoc_include);

    // Find all .proto files.
    let mut proto_files: Vec<PathBuf> = Vec::new();
    for entry in walkdir::WalkDir::new(proto_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "proto") {
            println!("cargo:rerun-if-changed={}", path.display());
            proto_files.push(path.to_path_buf());
        }
    }
    println!("cargo:rerun-if-changed={}", proto_root);

    // Compile the `.proto` files into Rust code.
    prost_build::Config::new()
        // Tell prost to generate a file descriptor set (for gRPC reflection).
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(&proto_files, &[proto_root])?;

    Ok(())
}
