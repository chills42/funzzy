use cli::Command;

use std::io::Write;
use std::fs::File;
use std::error::Error;

pub const DEFAULT_CONTENT: &'static str = "
## Funzzy events file
# more details see: https://github.com/cristianoliveira/funzzy
#
# list here all the events and the commands that it should execute

- name: run my test
  run: 'ls -a'
  when:
    change: 'src/**'
";


/// # `InitCommand`
///
/// Creates a funzzy yaml boilerplate.
///
pub struct InitCommand {
    pub file_name: &'static str,
}

impl Command for InitCommand {
    fn execute(&self) -> Result<(), String> {
        let mut yaml: File = match File::create(self.file_name) {
            Ok(f) => f,
            Err(err) => panic!("File wasn't created. Cause: {}", err),
        };

        if let Err(err) = yaml.write_all(DEFAULT_CONTENT.as_ref()) {
            return Err(String::from(err.description()));
        }

        Ok(())
    }
}
