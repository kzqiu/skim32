#[macro_use]
extern crate lazy_static;
extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod instructions;

static VERSION: &'static str = "1.0.0";

fn print_usage(prog: &str, opts: Options) {
    println!("gmips32 {}", VERSION);
    println!("");
    println!("Usage:");
    println!("{} [OPTION]... [FILE]...", prog);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let mut opts = Options::new();

    opts.optflag("h", "help", "display help and exit");
    opts.optflag("V", "version", "display current version");
    opts.optflag("n", "number", "number output lines");
    opts.optopt(
        "p",
        "peek",
        "[BEGIN:END] displays certain lines of file",
        "RANGE",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(m) => {
            panic!("{}", m.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return;
    }

    if matches.opt_present("V") {
        println!("gmips32 version: {}", VERSION);
        return;
    }

    if !matches.free.is_empty() {
        for f in matches.free.iter() {
            compile(&f);
        }
    } else {
        panic!("{}", "No files specified!");
    }
}

fn compile(path: &str) {
    if let Ok(lines) = read_lines(path) {
        let _new_file = Path::new("asdf.txt");
        for line in lines {
            if let Ok(ip) = line {
                // Do something
                let out = if ip.len() > 0 {
                    instructions::process_instruction(&ip)
                } else {
                    String::from("")
                };
                println!("{}", out);
            }
        }
    } else {
        panic!("{}", "File does not exist!");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
