use std::path::PathBuf;

use calculator::Op;
use clap::Parser;

mod calculator;

#[derive(Parser)]
struct Command {
    #[clap(value_parser(parse_operator))]
    op: Op,
    x: i32,
    y: i32,
    component: PathBuf,
}

fn parse_operator(val: &str) -> anyhow::Result<Op> {
    match val {
        "add" => Ok(Op::Add),
        // "sub" => Ok(Op::Sub),
        // "mul" => Ok(Op::Mul),
        // "div" => Ok(Op::Div),
        // "mod" => Ok(Op::Mod),
        _ => Err(anyhow::anyhow!("Invalid operator")),
    }
}

impl Command {
    fn run(self) -> anyhow::Result<()> {
        let res = calculator::calculate(self.component, self.op, self.x, self.y)?;
        println!("op: {:?}, x: {}, y: {}, result: {}", self.op, self.x, self.y, res);
        Ok(())
    }   
}

fn main() {
    Command::parse().run().unwrap();
}
