package(
    default_visibility = ["//visibility:public"],
)

load("@build_stack_rules_proto//cpp:cpp_grpc_library.bzl", "cpp_grpc_library")
load("@build_stack_rules_proto//python:python_grpc_library.bzl", "python_grpc_library")
load("@com_google_protobuf//:protobuf.bzl", "internal_copied_filegroup")

P4RUNTIME_PROTO_MAP = {
    "p4types": "p4/config/v1/p4types.proto",
    "p4info": "p4/config/v1/p4info.proto",
    "p4data": "p4/v1/p4data.proto",
    "p4runtime": "p4/v1/p4runtime.proto",
}

P4RUNTIME_PROTOS = ["proto/" + s for s in P4RUNTIME_PROTO_MAP.values()]

# Strip "proto" prefix from protos. Theya re copied to $GENDIR.
internal_copied_filegroup(
    name = "_copy_protos",
    srcs = P4RUNTIME_PROTOS,
    dest = "",
    strip_prefix = "proto",
    visibility = ["//visibility:private"],
)

proto_library(
    name = "p4types_proto",
    srcs = [
        "p4/config/v1/p4types.proto",
    ],
)

proto_library(
    name = "p4data_proto",
    srcs = ["p4/v1/p4data.proto"],
)

proto_library(
    name = "p4info_proto",
    srcs = [
        "p4/config/v1/p4info.proto",
    ],
    deps = [
        ":p4types_proto",
        "@com_google_protobuf//:descriptor_proto",
        "@com_google_protobuf//:any_proto",
    ],
)

proto_library(
    name = "p4runtime_proto",
    srcs = ["p4/v1/p4runtime.proto"],
    deps = [
        ":p4info_proto",
        ":p4data_proto",
        "@com_google_protobuf//:any_proto",
        "@com_google_googleapis//google/rpc:status_proto"
    ],
)

cc_proto_library(
    name = "p4types_cc_proto",
    deps = [":p4types_proto"],
)

cc_proto_library(
    name = "p4info_cc_proto",
    deps = [":p4info_proto"],
)

cc_proto_library(
    name = "p4data_cc_proto",
    deps = [":p4data_proto"],
)

cc_proto_library(
    name = "p4runtime_cc_proto",
    deps = [
        ":p4runtime_proto",
    ],
)

cpp_grpc_library(
    name = "p4runtime_cc_grpc",
    deps = [
        ":p4runtime_proto"
    ],
)

python_grpc_library(
    name = "p4runtime_py_grpc",
    deps = [":p4runtime_proto"],
)