use clap::{ ArgMatches, App, };
use crate::util::XResult;

pub trait Command {

    fn name(&self) -> &str;

    fn subcommand<'a>(&self) -> Option<App<'a, 'a>>;

    fn run(&self, arg_matches: &ArgMatches, sub_arg_matches: &ArgMatches) -> XResult<()>;
}
