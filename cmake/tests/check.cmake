# SPDX-FileCopyrightText: 2026 The P4 Language Consortium
#
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.16)
if(NOT MODE MATCHES "^(schemas|cpp|grpc)$" OR NOT TEST_BINARY_DIR)
  message(FATAL_ERROR "Set MODE to schemas/cpp/grpc and TEST_BINARY_DIR to a scratch directory")
endif()
get_filename_component(source "${CMAKE_CURRENT_LIST_DIR}/../.." ABSOLUTE)
get_filename_component(TEST_BINARY_DIR "${TEST_BINARY_DIR}" ABSOLUTE)
function(run)
  execute_process(COMMAND ${ARGV} RESULT_VARIABLE result)
  if(NOT result EQUAL 0)
    message(FATAL_ERROR "Command failed (${result}): ${ARGV}")
  endif()
endfunction()
set(cpp OFF)
set(grpc OFF)
if(NOT MODE STREQUAL "schemas")
  set(cpp ON)
endif()
if(MODE STREQUAL "grpc")
  set(grpc ON)
endif()
set(build "${TEST_BINARY_DIR}/build")
set(prefix "${TEST_BINARY_DIR}/prefix")
set(relocated "${TEST_BINARY_DIR}/relocated")
run("${CMAKE_COMMAND}" -S "${source}" -B "${build}"
  -DP4RUNTIME_BUILD_CPP=${cpp} -DP4RUNTIME_BUILD_GRPC=${grpc}
  -DCMAKE_BUILD_TYPE=Release)
run("${CMAKE_COMMAND}" --build "${build}" --config Release --parallel 2)
run("${CMAKE_COMMAND}" --install "${build}" --config Release --prefix "${prefix}")
file(REMOVE_RECURSE "${relocated}")
file(RENAME "${prefix}" "${relocated}")

# Even an installation containing libraries must support schemas without
# discovering Protobuf or gRPC. Also exercise install-prefix relocation.
run("${CMAKE_COMMAND}" -S "${source}/cmake/tests/consumer" -B "${TEST_BINARY_DIR}/schemas"
  -DCOMPONENT=schemas "-DCMAKE_PREFIX_PATH=${relocated}"
  -DCMAKE_DISABLE_FIND_PACKAGE_Protobuf=ON -DCMAKE_DISABLE_FIND_PACKAGE_gRPC=ON)
run("${CMAKE_COMMAND}" -S "${source}/cmake/tests/consumer" -B "${TEST_BINARY_DIR}/installed"
  -DCOMPONENT=${MODE} "-DCMAKE_PREFIX_PATH=${relocated}")
run("${CMAKE_COMMAND}" --build "${TEST_BINARY_DIR}/installed" --config Release --parallel 2)
run("${CMAKE_COMMAND}" -S "${source}/cmake/tests/consumer" -B "${TEST_BINARY_DIR}/subproject"
  -DCOMPONENT=${MODE} "-DP4RUNTIME_SOURCE_DIR=${source}")
run("${CMAKE_COMMAND}" --build "${TEST_BINARY_DIR}/subproject" --config Release --parallel 2)
if(NOT MODE STREQUAL "schemas")
  find_program(ctest NAMES ctest)
  if(NOT ctest)
    message(FATAL_ERROR "ctest is required to run the consumers")
  endif()
  foreach(consumer installed subproject)
    execute_process(COMMAND "${ctest}" -C Release --output-on-failure
      WORKING_DIRECTORY "${TEST_BINARY_DIR}/${consumer}" RESULT_VARIABLE result)
    if(NOT result EQUAL 0)
      message(FATAL_ERROR "${consumer} consumer failed")
    endif()
  endforeach()
else()
  execute_process(COMMAND "${CMAKE_COMMAND}" -S "${source}/cmake/tests/consumer"
    -B "${TEST_BINARY_DIR}/missing-cpp" -DCOMPONENT=cpp "-DCMAKE_PREFIX_PATH=${relocated}"
    RESULT_VARIABLE result OUTPUT_VARIABLE output ERROR_VARIABLE error)
  if(result EQUAL 0 OR NOT "${output}${error}" MATCHES "p4runtime_FOUND.*FALSE")
    message(FATAL_ERROR "A required unavailable component must fail: ${output}${error}")
  endif()
endif()
