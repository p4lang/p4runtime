// Copyright 2025 Steffen Smolka
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define constants.
    let protoc_version = "33.0"; // Version of the Protobuf compiler to use.
    let proto_root = "../../proto";

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
    tonic_prost_build::configure()
        // Tell tonic to use the prost-generated types for the Protobuf messages.
        .extern_path(".p4", "::p4runtime_prost::p4")
        // Build server and client code for the gRPC service.
        .build_server(true)
        .build_client(true)
        .compile_protos(&proto_files, &[PathBuf::from(proto_root)])?;

    Ok(())
}
