use clap::{ ArgMatches, SubCommand, App, };
use crate::util::XResult;
use crate::cmd::Command;

pub struct CommandDidCocument {
}

impl Command for CommandDidCocument {

    fn name(&self) -> &str { "did_document" }

    fn subcommand<'a>(&self) -> Option<App<'a, 'a>> {
        Some(SubCommand::with_name(self.name()).about("DID document"))
    }

    fn run(&self, _arg_matches: &ArgMatches, _sub_arg_matches: &ArgMatches) -> XResult<()> {
        println!("This is test command!");
        Ok(())
    }
}