// Copyright [2020] [Donatien Criaud]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::command::Command;
use crate::core::Owl;
use std::fmt;

pub struct PopCommand {
  key: String,
}

impl Command for PopCommand {
  fn new(args: Vec<&str>) -> Self {
    if args.len() != 1 {
      panic!(
        "Insert command requires exactly one arguments, {} were provided",
        args.len()
      );
    }
    PopCommand {
      key: args.get(0).unwrap().to_string(),
    }
  }
  fn execute(&self, engine: &mut Owl) -> Box<dyn fmt::Display> {
    match engine.pop(&self.key) {
      Ok(egg) => Box::new(egg),
      Err(error) => Box::new(error),
    }
  }
}