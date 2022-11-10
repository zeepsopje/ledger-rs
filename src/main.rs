pub mod ledger;
pub mod expense;

use std::env;
use ledger::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ledger = Ledger::new();

    match args.len() {
        1 => {
            println!("Balance: {}", args[0]);
        }
        2 => {
            let cmd = &args[1];

            match &cmd[..] {
                "help" => println!("help will be printed"),
                "in" => {
                    eprintln!("'in' expects a value.");
                },
                "out" => {
                    eprintln!("'out' expects a value.");
                },
                &_ => eprintln!("Command \"{}\" not recognized.", cmd)
            }
        },
        3 => {
            let cmd = &args[1];
            let val = &args[2];

            match &cmd[..] {
                "in" => {
                    match val.parse() {
                        Ok(num) => ledger.deposit(num),
                        Err(_) => eprintln!("'in' expects a numerical value")
                    }
                },
                "out" => {
                    match val.parse() {
                        Ok(num) => ledger.withdraw(num),
                        Err(_) => eprintln!("'out' expects a numerical value")
                    }
                },
                &_ => eprintln!("Command \"{}\" not recognized.", cmd)
            }
        }
        _ => eprintln!("Too many arguments")
    }

    println!("{}", ledger.balance);
}
