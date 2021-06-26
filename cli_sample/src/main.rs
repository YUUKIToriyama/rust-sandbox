use clap::{App, Arg};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
mod lib;

fn main() {
    let matches = App::new("cli sample")
        .version("0.9.0")
        .author("Y.Toriyama")
        .arg(
            Arg::new("formula")
                .about("数式")
                .short('f')
                .long("formula")
                .value_name("FORMULA")
                .required(false),
        )
        .arg(
            Arg::new("input")
                .about("ファイル入力")
                .short('i')
                .long("input-file")
                .value_name("FILE")
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("詳細を表示します")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("input") {
        // ファイル名が指定されている場合
        Some(filename) => {
            let f = File::open(filename).unwrap();
            let reader = BufReader::new(f);
            run(reader, matches.is_present("verbose"));
        }
        // 指定されていない場合
        None => {
            let stdin = stdin();
            let reader = stdin.lock();
            run(reader, matches.is_present("verbose"));
        }
    }
}

// R: BufReadを実装している任意の型。BufReader<File>型、StdinLock型を想定
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calculator = lib::RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let result = calculator.eval(&line);
        println!("{}", result);
    }
}
