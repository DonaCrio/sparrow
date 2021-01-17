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

use super::{CommandNotFoundError, CommandNotParsableError, PoisonedQueueError};

pub type Result<T> = std::result::Result<T, SparrowError>;

#[derive(Debug)]
pub enum SparrowError {
  CommandNotFound(CommandNotFoundError),
  CommandNotParsable(CommandNotParsableError),
  IOError(std::io::Error),
  PoisonedQueue(PoisonedQueueError),
}

impl std::error::Error for SparrowError {}

impl std::fmt::Display for SparrowError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      SparrowError::CommandNotFound(ref inner) => inner.fmt(f),
      SparrowError::CommandNotParsable(ref inner) => inner.fmt(f),
      SparrowError::IOError(ref inner) => inner.fmt(f),
      SparrowError::PoisonedQueue(ref inner) => inner.fmt(f),
    }
  }
}

impl From<CommandNotFoundError> for SparrowError {
  fn from(err: CommandNotFoundError) -> SparrowError {
    SparrowError::CommandNotFound(err)
  }
}

impl From<CommandNotParsableError> for SparrowError {
  fn from(err: CommandNotParsableError) -> SparrowError {
    SparrowError::CommandNotParsable(err)
  }
}

impl From<std::io::Error> for SparrowError {
  fn from(err: std::io::Error) -> SparrowError {
    SparrowError::IOError(err)
  }
}

impl From<PoisonedQueueError> for SparrowError {
  fn from(err: PoisonedQueueError) -> SparrowError {
    SparrowError::PoisonedQueue(err)
  }
}