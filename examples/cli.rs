extern crate laindb;
use std::io::{self, Write};
use laindb::Laindb;

fn main() {
    let name = match std::env::args().nth(1) {
        Some(name) => name,
        None => {
            println!("database name expected");
            std::process::exit(1);
        }
    };
    let db = Laindb::new(&name, laindb::Mode::Create);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let command = line.next().unwrap();
        match command {
            "GET" => {
                let key = line.next().unwrap();
                match db.get(key) {
                    Some(val) => {
                        let val = String::from_utf8(val).unwrap();
                        println!("-> {}", val);
                    }
                    None => println!("-> (Nothing)"),
                }
            }
            "DEL" => {
                let key = line.next().unwrap();
                db.erase(key);
            }
            "PUT" => {
                let key = line.next().unwrap();
                let val = line.next().unwrap();
                db.put(key, &val.to_string().into_bytes());
            }
            "EXIT" => break,
            _ => println!("Unknown operation"),
        }
    }
}