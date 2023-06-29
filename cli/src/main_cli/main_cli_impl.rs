use super::{MainCli, Commands};
use command::interface::{CommandInit, CommandAction};
use utils::prelude::init;

impl CommandInit for MainCli {
    fn init(&mut self) {
        init(self.debug);

        match &mut self.command {
            Commands::Init(init)=>{
                init.action();
            }
        }
    }
}