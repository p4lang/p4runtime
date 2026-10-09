# SPDX-FileCopyrightText: 2026 The P4 Language Consortium
#
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.16)
project(p4runtime VERSION 1.6.0 LANGUAGES NONE)

# Installing a subproject should be an explicit parent-project choice.
if(CMAKE_VERSION VERSION_LESS 3.21)
  set(p4runtime_IS_TOP_LEVEL OFF)
  if(CMAKE_SOURCE_DIR STREQUAL PROJECT_SOURCE_DIR)
    set(p4runtime_IS_TOP_LEVEL ON)
  endif()
endif()
option(P4RUNTIME_INSTALL "Install P4Runtime schemas, libraries, and CMake package" ${p4runtime_IS_TOP_LEVEL})

option(P4RUNTIME_BUILD_CPP "Build C++ Protobuf message bindings" OFF)
option(P4RUNTIME_BUILD_GRPC "Build C++ gRPC service bindings (requires P4RUNTIME_BUILD_CPP)" OFF)
if(P4RUNTIME_BUILD_GRPC AND NOT P4RUNTIME_BUILD_CPP)
  message(FATAL_ERROR "P4RUNTIME_BUILD_GRPC requires P4RUNTIME_BUILD_CPP=ON")
endif()

# Schema-only builds do not require a compiler, Protobuf, or gRPC.
if(P4RUNTIME_BUILD_CPP)
  enable_language(CXX)
endif()
# GNUInstallDirs needs an architecture only for libraries. Schemas and the
# package configuration are architecture-independent, including cross builds.
if(P4RUNTIME_BUILD_CPP)
  include(GNUInstallDirs)
else()
  set(CMAKE_INSTALL_DATADIR share CACHE PATH "Read-only architecture-independent data")
endif()
include(CMakePackageConfigHelpers)
set(p4runtime_INSTALL_PROTO_DIR "${CMAKE_INSTALL_DATADIR}/p4runtime/proto")
set(p4runtime_INSTALL_CMAKE_DIR "${CMAKE_INSTALL_DATADIR}/cmake/p4runtime")

add_library(p4runtime_schemas INTERFACE)
add_library(p4runtime::schemas ALIAS p4runtime_schemas)
set_target_properties(p4runtime_schemas PROPERTIES EXPORT_NAME schemas)
target_include_directories(p4runtime_schemas INTERFACE
  "$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/proto>"
  "$<INSTALL_INTERFACE:${p4runtime_INSTALL_PROTO_DIR}>")
if(P4RUNTIME_INSTALL)
  install(DIRECTORY proto/ DESTINATION "${p4runtime_INSTALL_PROTO_DIR}"
    FILES_MATCHING PATTERN "*.proto")
  install(TARGETS p4runtime_schemas EXPORT p4runtimeSchemas)
  install(EXPORT p4runtimeSchemas NAMESPACE p4runtime::
    DESTINATION "${p4runtime_INSTALL_CMAKE_DIR}")
endif()

if(P4RUNTIME_BUILD_CPP)
  include(cmake/Cpp.cmake)
endif()

if(P4RUNTIME_INSTALL)
  configure_package_config_file(cmake/p4runtimeConfig.cmake.in
    "${CMAKE_CURRENT_BINARY_DIR}/p4runtimeConfig.cmake"
    INSTALL_DESTINATION "${p4runtime_INSTALL_CMAKE_DIR}"
    PATH_VARS p4runtime_INSTALL_PROTO_DIR)
  write_basic_package_version_file("${CMAKE_CURRENT_BINARY_DIR}/p4runtimeConfigVersion.cmake"
    VERSION "${PROJECT_VERSION}" COMPATIBILITY SameMajorVersion ARCH_INDEPENDENT)
  install(FILES "${CMAKE_CURRENT_BINARY_DIR}/p4runtimeConfig.cmake"
                "${CMAKE_CURRENT_BINARY_DIR}/p4runtimeConfigVersion.cmake"
    DESTINATION "${p4runtime_INSTALL_CMAKE_DIR}")
endif()
