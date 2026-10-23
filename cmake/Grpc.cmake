# SPDX-FileCopyrightText: 2026 The P4 Language Consortium
#
# SPDX-License-Identifier: Apache-2.0

# Cpp.cmake provides protoc, import paths, and the message library.
if(NOT TARGET gRPC::grpc++)
  find_package(gRPC CONFIG REQUIRED)
endif()

set(P4RUNTIME_GRPC_CPP_PLUGIN "" CACHE FILEPATH
  "Host grpc_cpp_plugin executable; defaults to the gRPC::grpc_cpp_plugin target"
)
if(P4RUNTIME_GRPC_CPP_PLUGIN)
  set(p4runtime_grpc_plugin "${P4RUNTIME_GRPC_CPP_PLUGIN}")
elseif(TARGET gRPC::grpc_cpp_plugin AND NOT CMAKE_CROSSCOMPILING)
  set(p4runtime_grpc_plugin "$<TARGET_FILE:gRPC::grpc_cpp_plugin>")
else()
  message(FATAL_ERROR "Set P4RUNTIME_GRPC_CPP_PLUGIN to a host grpc_cpp_plugin")
endif()

set(p4runtime_grpc_outputs
  "${p4runtime_generated_dir}/p4/v1/p4runtime.grpc.pb.cc"
  "${p4runtime_generated_dir}/p4/v1/p4runtime.grpc.pb.h"
)

add_custom_command(
  OUTPUT ${p4runtime_grpc_outputs}
  COMMAND ${p4runtime_protoc}
    "--grpc_out=${p4runtime_generated_dir}"
    "--plugin=protoc-gen-grpc=${p4runtime_grpc_plugin}"
    "--proto_path=${p4runtime_proto_dir}"
    "${p4runtime_protobuf_include_args}"
    "${p4runtime_proto_dir}/p4/v1/p4runtime.proto"
  DEPENDS
    ${p4runtime_proto_inputs}
    ${p4runtime_protoc}
    "${p4runtime_grpc_plugin}"
  COMMENT "Generating P4Runtime C++ gRPC service bindings"
  COMMAND_EXPAND_LISTS
  VERBATIM
)

add_library(p4runtime_grpc STATIC ${p4runtime_grpc_outputs})
add_library(p4runtime::grpc ALIAS p4runtime_grpc)

set_target_properties(p4runtime_grpc PROPERTIES
  EXPORT_NAME grpc
  POSITION_INDEPENDENT_CODE ON
)
target_link_libraries(p4runtime_grpc PUBLIC p4runtime_cpp gRPC::grpc++)

if(P4RUNTIME_INSTALL)
  install(TARGETS p4runtime_grpc EXPORT p4runtimeGrpc
    ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}"
  )
  install(EXPORT p4runtimeGrpc
    NAMESPACE p4runtime::
    DESTINATION "${p4runtime_INSTALL_CMAKE_DIR}"
  )
endif()
