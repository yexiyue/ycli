mod main_cli;
use clap::Parser;
use command::interface::CommandInit;
use main_cli::MainCli;

pub fn init() {
    let mut cli=MainCli::parse();
    cli.init();
}