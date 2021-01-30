use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().map(|a| a.to_string()).collect();
    let n: u32 = u32::from_str(args[1].as_str()).unwrap();
    let r = cmp_error::do_anyhow(n);
    println!("{:?}", r);
}
