mod converter;

fn main() {
    let str = converter::han2zen("ﾄﾘﾔﾏﾕｳｷ".to_string());
    println!("{}", str);
}
