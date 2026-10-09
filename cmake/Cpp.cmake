# SPDX-FileCopyrightText: 2026 The P4 Language Consortium
#
# SPDX-License-Identifier: Apache-2.0

# Reuse dependencies provided by a parent project (including CPM). Prefer
# upstream package configs, with FindProtobuf for distribution packages.
if(NOT TARGET protobuf::libprotobuf)
  find_package(Protobuf CONFIG QUIET)
  if(NOT Protobuf_FOUND)
    find_package(Protobuf REQUIRED MODULE)
  endif()
endif()
set(P4RUNTIME_PROTOC_EXECUTABLE "" CACHE FILEPATH
  "Host protoc executable; defaults to the protobuf::protoc target")
if(P4RUNTIME_PROTOC_EXECUTABLE)
  set(p4runtime_protoc "${P4RUNTIME_PROTOC_EXECUTABLE}")
elseif(TARGET protobuf::protoc AND NOT CMAKE_CROSSCOMPILING)
  set(p4runtime_protoc protobuf::protoc)
else()
  message(FATAL_ERROR "Set P4RUNTIME_PROTOC_EXECUTABLE to a host protoc matching the Protobuf library")
endif()

set(p4runtime_proto_files
  google/rpc/status.proto
  p4/config/v1/p4info.proto
  p4/config/v1/p4types.proto
  p4/v1/p4data.proto
  p4/v1/p4runtime.proto)
set(p4runtime_generated_dir "${CMAKE_CURRENT_BINARY_DIR}/generated")
file(MAKE_DIRECTORY "${p4runtime_generated_dir}")
set(p4runtime_proto_inputs)
set(p4runtime_cpp_outputs)
foreach(proto IN LISTS p4runtime_proto_files)
  list(APPEND p4runtime_proto_inputs "${CMAKE_CURRENT_SOURCE_DIR}/proto/${proto}")
  string(REGEX REPLACE "\\.proto$" ".pb" stem "${proto}")
  list(APPEND p4runtime_cpp_outputs
    "${p4runtime_generated_dir}/${stem}.cc" "${p4runtime_generated_dir}/${stem}.h")
endforeach()
# Include paths from the target also work with parent-provided Protobuf. This
# supplies standard imports such as google/protobuf/any.proto.
set(p4runtime_protobuf_includes "$<TARGET_PROPERTY:protobuf::libprotobuf,INTERFACE_INCLUDE_DIRECTORIES>")
add_custom_command(OUTPUT ${p4runtime_cpp_outputs}
  COMMAND ${p4runtime_protoc}
    "--cpp_out=${p4runtime_generated_dir}"
    "--proto_path=${CMAKE_CURRENT_SOURCE_DIR}/proto"
    "$<$<BOOL:${p4runtime_protobuf_includes}>:-I$<JOIN:${p4runtime_protobuf_includes},;-I>>"
    ${p4runtime_proto_inputs}
  DEPENDS ${p4runtime_proto_inputs} ${p4runtime_protoc}
  COMMENT "Generating P4Runtime C++ message bindings"
  COMMAND_EXPAND_LISTS VERBATIM)
add_library(p4runtime_cpp STATIC ${p4runtime_cpp_outputs})
add_library(p4runtime::cpp ALIAS p4runtime_cpp)
set_target_properties(p4runtime_cpp PROPERTIES EXPORT_NAME cpp POSITION_INDEPENDENT_CODE ON)
target_compile_features(p4runtime_cpp PUBLIC cxx_std_14)
target_include_directories(p4runtime_cpp PUBLIC
  "$<BUILD_INTERFACE:${p4runtime_generated_dir}>"
  "$<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>")
target_link_libraries(p4runtime_cpp PUBLIC protobuf::libprotobuf)
if(P4RUNTIME_INSTALL)
  install(TARGETS p4runtime_cpp EXPORT p4runtimeCpp
    ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}")
  install(EXPORT p4runtimeCpp NAMESPACE p4runtime:: DESTINATION "${p4runtime_INSTALL_CMAKE_DIR}")
endif()


if(P4RUNTIME_BUILD_GRPC)
  if(NOT TARGET gRPC::grpc++)
    find_package(gRPC CONFIG REQUIRED)
  endif()
  set(P4RUNTIME_GRPC_CPP_PLUGIN "" CACHE FILEPATH
    "Host grpc_cpp_plugin executable; defaults to the gRPC::grpc_cpp_plugin target")
  if(P4RUNTIME_GRPC_CPP_PLUGIN)
    set(p4runtime_grpc_plugin "${P4RUNTIME_GRPC_CPP_PLUGIN}")
  elseif(TARGET gRPC::grpc_cpp_plugin AND NOT CMAKE_CROSSCOMPILING)
    set(p4runtime_grpc_plugin "$<TARGET_FILE:gRPC::grpc_cpp_plugin>")
  else()
    message(FATAL_ERROR "Set P4RUNTIME_GRPC_CPP_PLUGIN to a host grpc_cpp_plugin")
  endif()
  set(p4runtime_grpc_outputs
    "${p4runtime_generated_dir}/p4/v1/p4runtime.grpc.pb.cc"
    "${p4runtime_generated_dir}/p4/v1/p4runtime.grpc.pb.h")
  add_custom_command(OUTPUT ${p4runtime_grpc_outputs}
    COMMAND ${p4runtime_protoc}
      "--grpc_out=${p4runtime_generated_dir}"
      "--plugin=protoc-gen-grpc=${p4runtime_grpc_plugin}"
      "--proto_path=${CMAKE_CURRENT_SOURCE_DIR}/proto"
      "$<$<BOOL:${p4runtime_protobuf_includes}>:-I$<JOIN:${p4runtime_protobuf_includes},;-I>>"
      "${CMAKE_CURRENT_SOURCE_DIR}/proto/p4/v1/p4runtime.proto"
    DEPENDS ${p4runtime_proto_inputs} ${p4runtime_protoc} "${p4runtime_grpc_plugin}"
    COMMENT "Generating P4Runtime C++ gRPC service bindings"
    COMMAND_EXPAND_LISTS VERBATIM)
  add_library(p4runtime_grpc STATIC ${p4runtime_grpc_outputs})
  add_library(p4runtime::grpc ALIAS p4runtime_grpc)
  set_target_properties(p4runtime_grpc PROPERTIES EXPORT_NAME grpc POSITION_INDEPENDENT_CODE ON)
  target_link_libraries(p4runtime_grpc PUBLIC p4runtime_cpp gRPC::grpc++)
  if(P4RUNTIME_INSTALL)
    install(TARGETS p4runtime_grpc EXPORT p4runtimeGrpc
      ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}")
    install(EXPORT p4runtimeGrpc NAMESPACE p4runtime:: DESTINATION "${p4runtime_INSTALL_CMAKE_DIR}")
  endif()

endif()
if(P4RUNTIME_INSTALL)
  install(DIRECTORY "${p4runtime_generated_dir}/" DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
    FILES_MATCHING PATTERN "*.h")
endif()
