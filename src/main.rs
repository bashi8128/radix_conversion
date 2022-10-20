//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Sun, 29 Sep 2019 22:07:00 +0900

use clap::Parser;

#[derive(clap::ValueEnum, Clone)]
enum PossibleBase {
    Bin,
    Oct,
    Dec,
    Hex,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = PossibleBase::Hex)]
    in_base: PossibleBase,

    #[arg(short, long, value_enum, default_value_t = PossibleBase::Dec)]
    out_base: PossibleBase,

    num: String,
}

fn main() {
    let args = Args::parse();

    let input_base = match args.in_base {
        PossibleBase::Bin => 2,
        PossibleBase::Oct => 8,
        PossibleBase::Dec => 10,
        PossibleBase::Hex => 16,
    };

    match i64::from_str_radix(&args.num, input_base) {
        Ok(i) => {
            match args.out_base {
                PossibleBase::Bin => println!("{:b}", i),
                PossibleBase::Oct => println!("{:o}", i),
                PossibleBase::Dec => println!("{}", i),
                PossibleBase::Hex => println!("{:x}", i),
            };
        }
        Err(e) => {
            eprintln!("Failed to convert NUM into numerical value: {:}", e);
        }
    };
}
