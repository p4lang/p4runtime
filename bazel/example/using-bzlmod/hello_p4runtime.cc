// Copyright 2024 Steffen Smolka
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

#include <iostream>

#include "google/protobuf/text_format.h"
#include "p4/config/v1/p4info.pb.h"

using ::google::protobuf::TextFormat;
using ::p4::config::v1::P4Info;

int main() {
  P4Info p4info;
  TextFormat::ParseFromString(R"PROTO(
                                  tables {
                                    preamble {
                                      id: 10
                                      name: "Hello, P4Runtime!"
                                    }
                                  }
                                )PROTO", &p4info);
  p4info.mutable_tables()->at(0).mutable_preamble()->set_id(42);
  std::cout << p4info.DebugString();
}
