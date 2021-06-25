use clap::{App, Arg};

fn main() {
    let matches = App::new("cli sample")
        .version("0.9.0")
        .author("Y.Toriyama")
        .arg(
            Arg::new("message")
                .about("メッセージ")
                .short('m')
                .long("message")
                .value_name("MESSAGE")
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("詳細を表示します")
                .short('v')
                .long("verbose")
                .required(false),
        );
    let parsed = matches.get_matches();
    match parsed.value_of("message") {
        Some(message) => println!("{}", message),
        None => println!("No message"),
    }

    if parsed.is_present("verbose") {
        println!("Verbose mode.");
    }
}
