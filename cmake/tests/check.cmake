# SPDX-FileCopyrightText: 2026 The P4 Language Consortium
#
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.16)

if(NOT MODE MATCHES "^(schemas|cpp|grpc)$" OR NOT TEST_BINARY_DIR)
  message(FATAL_ERROR
    "Set MODE to schemas/cpp/grpc and TEST_BINARY_DIR to a scratch directory"
  )
endif()

get_filename_component(source_dir "${CMAKE_CURRENT_LIST_DIR}/../.." ABSOLUTE)
get_filename_component(TEST_BINARY_DIR "${TEST_BINARY_DIR}" ABSOLUTE)
set(consumer_source_dir "${CMAKE_CURRENT_LIST_DIR}/consumer")
set(build_dir "${TEST_BINARY_DIR}/build")
set(install_prefix "${TEST_BINARY_DIR}/prefix")
set(relocated_prefix "${TEST_BINARY_DIR}/relocated")

function(run)
  execute_process(COMMAND ${ARGV} RESULT_VARIABLE result)
  if(NOT "${result}" STREQUAL "0")
    message(FATAL_ERROR "Command failed (${result}): ${ARGV}")
  endif()
endfunction()

function(configure_consumer name component)
  run("${CMAKE_COMMAND}"
    -S "${consumer_source_dir}"
    -B "${TEST_BINARY_DIR}/${name}"
    -DCOMPONENT=${component}
    ${ARGN}
  )
endfunction()

set(build_cpp OFF)
set(build_grpc OFF)
if(NOT MODE STREQUAL "schemas")
  set(build_cpp ON)
endif()
if(MODE STREQUAL "grpc")
  set(build_grpc ON)
endif()

# Build, install, and relocate the package before configuring consumers.
run("${CMAKE_COMMAND}"
  -S "${source_dir}"
  -B "${build_dir}"
  -DP4RUNTIME_BUILD_CPP=${build_cpp}
  -DP4RUNTIME_BUILD_GRPC=${build_grpc}
  -DCMAKE_BUILD_TYPE=Release
)
run("${CMAKE_COMMAND}" --build "${build_dir}" --config Release --parallel 2)
run("${CMAKE_COMMAND}" --install "${build_dir}"
  --config Release
  --prefix "${install_prefix}"
)

file(REMOVE_RECURSE "${relocated_prefix}")
file(RENAME "${install_prefix}" "${relocated_prefix}")

# Even an installation containing libraries must support schemas without
# discovering Protobuf or gRPC. Also exercise install-prefix relocation.
configure_consumer(schemas schemas
  "-DCMAKE_PREFIX_PATH=${relocated_prefix}"
  -DCMAKE_DISABLE_FIND_PACKAGE_Protobuf=ON
  -DCMAKE_DISABLE_FIND_PACKAGE_gRPC=ON
)
configure_consumer(installed ${MODE} "-DCMAKE_PREFIX_PATH=${relocated_prefix}")
configure_consumer(subproject ${MODE} "-DP4RUNTIME_SOURCE_DIR=${source_dir}")

foreach(consumer installed subproject)
  set(consumer_build_dir "${TEST_BINARY_DIR}/${consumer}")
  run("${CMAKE_COMMAND}" --build "${consumer_build_dir}"
    --config Release
    --parallel 2
  )

  if(build_cpp)
    run("${CMAKE_COMMAND}" -E chdir "${consumer_build_dir}"
      "${CMAKE_CTEST_COMMAND}" -C Release --output-on-failure
    )
  endif()
endforeach()

# A schema-only installation must reject a required C++ component.
if(NOT build_cpp)
  execute_process(
    COMMAND "${CMAKE_COMMAND}"
      -S "${consumer_source_dir}"
      -B "${TEST_BINARY_DIR}/missing-cpp"
      -DCOMPONENT=cpp
      "-DCMAKE_PREFIX_PATH=${relocated_prefix}"
    RESULT_VARIABLE result
    OUTPUT_VARIABLE output
    ERROR_VARIABLE error
  )
  if(result EQUAL 0 OR NOT "${output}${error}" MATCHES "p4runtime_FOUND.*FALSE")
    message(FATAL_ERROR "A required unavailable component must fail: ${output}${error}")
  endif()
endif()
