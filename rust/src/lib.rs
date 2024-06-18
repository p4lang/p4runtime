// @generated
pub mod google {
    #[cfg(feature = "google-rpc")]
    // @@protoc_insertion_point(attribute:google.rpc)
    pub mod rpc {
        include!("google.rpc.rs");
        // @@protoc_insertion_point(google.rpc)
    }
}
pub mod p4 {
    pub mod config {
        #[cfg(feature = "p4-config-v1")]
        // @@protoc_insertion_point(attribute:p4.config.v1)
        pub mod v1 {
            include!("p4.config.v1.rs");
            // @@protoc_insertion_point(p4.config.v1)
        }
    }
    #[cfg(feature = "p4-v1")]
    // @@protoc_insertion_point(attribute:p4.v1)
    pub mod v1 {
        include!("p4.v1.rs");
        // @@protoc_insertion_point(p4.v1)
    }
}
