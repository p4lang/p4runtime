package(
    default_visibility = ["//visibility:public"],
)

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

load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cpp_proto_library")

cpp_proto_library(
  name = "p4types_proto_cc",
  protos = ["p4/config/v1/p4types.proto"],
  with_grpc = False,
)

cpp_proto_library(
  name = "p4info_proto_cc",
  protos = ["p4/config/v1/p4info.proto"],
  proto_deps = [":p4types_proto_cc"],
  imports = ["external/com_google_protobuf/src/"],
  inputs = ["@com_google_protobuf//:well_known_protos"],
  with_grpc = False,
)

cpp_proto_library(
  name = "p4data_proto_cc",
  protos = ["p4/v1/p4data.proto"],
  with_grpc = False,
)

cpp_proto_library(
  name = "p4runtime_proto_cc",
  protos = ["p4/v1/p4runtime.proto"],
  proto_deps = [":p4info_proto_cc", ":p4data_proto_cc",
                "@com_github_googleapis//:status_proto_cc"],
  imports = ["external/com_google_protobuf/src/"],
  inputs = ["@com_google_protobuf//:well_known_protos"],
  with_grpc = False,
)

cpp_proto_library(
  name = "p4runtime_grpc_cc",
  protos = ["p4/v1/p4runtime.proto"],
  proto_deps = [":p4info_proto_cc", ":p4data_proto_cc",
                "@com_github_googleapis//:status_proto_cc"],
  imports = ["external/com_google_protobuf/src/"],
  inputs = ["@com_google_protobuf//:well_known_protos"],
  with_grpc = True,
)

load("@org_pubref_rules_protobuf//python:rules.bzl", "py_proto_compile")

# For some reasom, status_proto_py needs to be includes in both deps and
# inputs. If it is not listed in inputs, the Python file is not generated for
# status.proto.
py_proto_compile(
  name = "p4runtime_proto_py",
  protos = ["p4/config/v1/p4types.proto",
            "p4/config/v1/p4info.proto",
            "p4/v1/p4data.proto",
            "p4/v1/p4runtime.proto"],
  deps = ["@com_github_googleapis//:status_proto_py"],
  imports = ["external/com_google_protobuf/src/"],
  inputs = ["@com_google_protobuf//:well_known_protos",
            "@com_github_googleapis//:status_proto_py"],
  with_grpc = True,
)
