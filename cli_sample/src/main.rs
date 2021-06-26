use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        Some(filename) => load_file(&filename),
        None => println!("ファイルが指定されていません"),
    }

    if matches.is_present("verbose") {
        println!("Verbose mode.");
    }
}

fn load_file(path: &str) {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
