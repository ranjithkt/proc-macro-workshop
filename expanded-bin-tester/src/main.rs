#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use derive_builder::Builder;
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
pub struct CommandBuilder {
    executable: core::option::Option<String>,
    args: core::option::Option<Vec<String>>,
    env: core::option::Option<Vec<String>>,
    current_dir: core::option::Option<Option<String>>,
}
impl CommandBuilder {
    pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = Some(executable);
        self
    }
    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = Some(args);
        self
    }
    pub fn env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = Some(env);
        self
    }
    pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = Some(Some(current_dir));
        self
    }
    pub fn build(&mut self) -> Result<Command, std::boxed::Box<dyn std::error::Error>> {
        core::result::Result::Ok(Command {
            executable: self.executable.take().ok_or("executable not set")?,
            args: self.args.take().ok_or("args not set")?,
            env: self.env.take().ok_or("env not set")?,
            current_dir: self.current_dir.take().unwrap_or_default(),
        })
    }
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: None,
            args: None,
            env: None,
            current_dir: None,
        }
    }
}
fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(Vec::new())
        .build()
        .unwrap();
    if !command.current_dir.is_none() {
        panic!("assertion failed: command.current_dir.is_none()")
    }
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(Vec::new())
        .current_dir("..".to_owned())
        .build()
        .unwrap();
    if !command.current_dir.is_some() {
        panic!("assertion failed: command.current_dir.is_some()")
    }
}