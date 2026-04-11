use prost::Message;
use prost_types::FileDescriptorSet;

/// The official Prost API for all
/// [P4Runtime Protobuf messages](https://github.com/p4lang/p4runtime/blob/main/proto).
pub mod p4 {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/p4.v1.rs"));
    }
    pub mod config {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/p4.config.v1.rs"));
        }
    }
}

/// Unofficial Prost API for [`google/rpc/status.proto`](https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto).
///
/// Included as a workaround here since there is currently no official crate providing this API.
pub mod google {
    pub mod rpc {
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
    }
}

/// Returns a `FileDescriptorSet` defining all Protobuf messages used by P4Runtime.
///
/// Enables [gRPC reflection](https://grpc.io/docs/guides/reflection/).
pub fn file_descriptor_set() -> FileDescriptorSet {
    let raw_proto: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    return FileDescriptorSet::decode(raw_proto).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_descriptor_set_is_not_empty() {
        let descriptor_set = file_descriptor_set();
        assert!(
            !descriptor_set.file.is_empty(),
            "FileDescriptorSet is empty"
        );
    }
}
