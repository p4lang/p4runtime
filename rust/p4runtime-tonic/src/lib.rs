/// The official Tonic API for all P4Runtime gRPC services, including clients and server code.
pub mod p4 {
    pub mod v1 {
        tonic::include_proto!("p4.v1");
    }
}
