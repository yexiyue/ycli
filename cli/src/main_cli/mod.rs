use clap::{command,arg, Parser, Subcommand};
use init::InitArgs;
pub mod main_cli_impl;

#[derive(Debug,Parser)]
#[command(author,version,about,long_about = None)]
pub struct MainCli{
    ///是否开启调试模式
    #[arg(short,long,default_value="false")]
    debug:bool,

    #[command(subcommand)]
    command:Commands,
}

//子命令在这里进行注册
#[derive(Debug,Subcommand)]
enum Commands
{   
    ///初始化项目
    Init(InitArgs)
}