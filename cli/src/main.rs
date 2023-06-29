use ycli::init;
fn main() {
    init();
}


/* use clap::{Parser,command,arg, Subcommand, Args};

#[derive(Parser,Debug)]
#[command(author,version,about)]
struct Cli{
    #[arg(short,long,default_value="false")]
    /// 是否启动调试模式
    debug:bool,

    #[command(subcommand)]
    command:Commands
}
#[derive(Debug,Subcommand)]
enum Commands {
    Init(InitArgs),
}

#[derive(Args,Debug,Clone)]
#[group(required=true,multiple=true)]
struct InitArgs{
    name:Option<String>,

    #[arg(short,long,default_value="false")]
    force:bool
}

trait CommandAction {
    type CbParams;
    fn action(&self,cb:impl Send + Sync + Fn(Self::CbParams)->())->();
}

impl CommandAction for InitArgs {
    type CbParams =(String,bool);
    
    fn action<'b>(&self,cb:impl Fn((String,bool))->()){
        match &self.name {
            Some(name)=>{
                cb((name.clone(),self.force))
            },
            None=>()
        };
    }
} */
