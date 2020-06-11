use clap::{ ArgMatches, App, };
use crate::util::XResult;

pub trait Command {

    fn subcommand<'a>(&self) -> Option<App<'a, 'a>>;

    fn name(&self) -> &str;

    fn run(&self, arg_matches: &ArgMatches, sub_arg_matches: &ArgMatches) -> XResult<()>;
}