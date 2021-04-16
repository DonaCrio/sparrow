use crate::core::commands::Command;
use crate::core::egg::Egg;
use crate::core::errors::Result;
use crate::core::nest::Nest;
use std::fmt;

/// Engine POP command.
#[derive(Clone, Debug)]
pub struct PopCommand {
  key: String,
}

impl PopCommand {
  /// Return a new [`PopCommand`].
  ///
  /// # Arguments
  /// * `args` - Arguments of this command. There should be 1 argument (key).
  ///
  /// # Examples
  /// ```rust
  /// use sparrow::core::commands::PopCommand;
  ///
  /// let args = &vec!["my key"];
  /// let cmd = PopCommand::new(args).unwrap();
  ///
  /// assert_eq!(format!("{}", cmd), "POP {my key}");
  /// ```
  ///
  /// [`PopCommand`]: sparrow::core::commands::get_command::PopCommand
  pub fn new(args: &[&str]) -> Result<PopCommand> {
    match args.len() {
      1 => {
        let key = args.get(0).unwrap();
        Ok(PopCommand {
          key: key.to_string(),
        })
      }
      n => Err(
        format!(
          "Cannot parse POP command arguments: Wrong number of arguments. Expected 1, got {}.",
          n
        )
        .into(),
      ),
    }
  }
}

impl fmt::Display for PopCommand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "POP {{{}}}", self.key)
  }
}

impl Command for PopCommand {
  /// Execute the `POP key` command on a given [`Nest`].
  ///
  /// [`Nest`]: sparrow::core::nest::Nest
  fn execute(&self, nest: &mut Nest) -> Option<Egg> {
    nest.pop(&self.key)
  }
}

#[cfg(test)]
mod tests {
  use crate::core::commands::pop_command::PopCommand;
  use crate::core::commands::Command;
  use crate::core::egg::Egg;
  use crate::core::nest::Nest;
  use rstest::*;

  const TEST_KEY: &str = "My key";
  const TEST_VALUE: &str = "This is a test value!";

  #[fixture]
  fn nest() -> Nest {
    Nest::new()
  }

  #[test]
  fn test_command_new_1_args() {
    let args = &vec![TEST_KEY];
    let command = PopCommand::new(args).unwrap();
    assert_eq!(command.key, TEST_KEY)
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse POP command arguments: Wrong number of arguments. Expected 1, got 0."
  )]
  fn test_command_new_0_args() {
    let args = &vec![];
    PopCommand::new(args).unwrap();
  }

  #[test]
  #[should_panic(
    expected = "Cannot parse POP command arguments: Wrong number of arguments. Expected 1, got 2."
  )]
  fn test_command_new_2_args() {
    let args = &vec![TEST_KEY, TEST_VALUE];
    PopCommand::new(args).unwrap();
  }

  #[rstest]
  fn test_command_execute(mut nest: Nest) {
    let args = &vec![TEST_KEY];
    let command = Box::new(PopCommand::new(args).unwrap());

    let egg = command.execute(&mut nest);
    assert!(egg.is_none());

    nest.insert(Egg::new(TEST_KEY, TEST_VALUE));
    let egg = command.execute(&mut nest).unwrap();
    assert_eq!(egg.key(), TEST_KEY);
    assert_eq!(egg.value(), TEST_VALUE);

    let egg = nest.get(TEST_KEY);
    assert!(egg.is_none());
  }
}