use predicates::prelude::*;
use std::process::Command;
use console::style;
#[test]
fn test_help() {
    let output = Command::new("ycli")
        .arg("--help")
        .output()
        .expect("输出信息");
    println!("{}",style(String::from_utf8((&output.stdout).clone()).unwrap()).yellow());

    assert_eq!(
        true,
        predicate::str::contains("Usage: ycli [OPTIONS] <COMMAND>\n\nCommands:\n")
            .eval(&String::from_utf8((&output.stdout).clone()).unwrap())
    )
}