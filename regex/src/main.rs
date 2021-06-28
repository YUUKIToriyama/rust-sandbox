use regex::Regex;

fn main() {
    let regular_expression =
        Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let captures = regular_expression.captures("2021-06-21").unwrap();
    println!("{:?}", &captures["year"]);
}
