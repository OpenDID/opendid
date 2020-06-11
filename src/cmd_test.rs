use clap::{ ArgMatches, SubCommand, App, };
use crate::util::XResult;
use crate::cmd::Command;

pub struct CommandTest {
}

impl Command for CommandTest {

    fn subcommand<'a>(&self) -> Option<App<'a, 'a>> {
        Some(SubCommand::with_name(self.name()).about("test"))
    }

    fn name(&self) -> &str {
        "test"
    }

    fn run(&self, _arg_matches: &ArgMatches, _sub_arg_matches: &ArgMatches) -> XResult<()> {
        println!("This is test command!");
        Ok(())
    }
}