//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Mon, 24 Oct 2022 04:23:07 AM +0900.

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
    /// Regard input NUM as IN_BASE based number.
    #[arg(short, long, value_enum, default_value_t = PossibleBase::Hex)]
    in_base: PossibleBase,

    /// Convert input NUM into OUT_BASE based number.
    #[arg(short, long, value_enum, default_value_t = PossibleBase::Dec)]
    out_base: PossibleBase,

    /// If specified, the input NUM will be converted into all possible bases
    #[arg(short, long, default_value_t = false)]
    all_out: bool,

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
            if args.all_out {
                println!("Bin: {:b}", i);
                println!("Oct: {:o}", i);
                println!("Dec: {}", i);
                println!("Hex: {:x}", i);
            } else {
                match args.out_base {
                    PossibleBase::Bin => println!("{:b}", i),
                    PossibleBase::Oct => println!("{:o}", i),
                    PossibleBase::Dec => println!("{}", i),
                    PossibleBase::Hex => println!("{:x}", i),
                };
            }
        }
        Err(e) => {
            eprintln!("Failed to convert NUM into numerical value: {:}", e);
        }
    };
}
