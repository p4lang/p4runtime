// SPDX-FileCopyrightText: 2026 The P4 Language Consortium
//
// SPDX-License-Identifier: Apache-2.0

#include <p4/config/v1/p4info.pb.h>
#include <p4/v1/p4runtime.pb.h>
#ifdef TEST_GRPC
#include <grpcpp/grpcpp.h>
#include <p4/v1/p4runtime.grpc.pb.h>
#endif

int main() {
    p4::v1::WriteRequest request;
    request.set_device_id(42);
    p4::v1::WriteRequest decoded;
    if (!decoded.ParseFromString(request.SerializeAsString()) || decoded.device_id() != 42)
        return 1;
    p4::config::v1::P4Info info;
    info.mutable_pkg_info()->set_name("cmake-consumer");
#ifdef TEST_GRPC
    auto stub = p4::v1::P4Runtime::NewStub(
        grpc::CreateChannel("localhost:9559", grpc::InsecureChannelCredentials()));
    if (!stub) return 1;
#endif
    return info.pkg_info().name() == "cmake-consumer" ? 0 : 1;
}
