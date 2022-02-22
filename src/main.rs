use std::fs::File;
use std::io::*;
mod lib;
use lib::gen_rand;
fn main() {
    println!("Program executed \n");

    let reg = "+0 000 "; //enter country and region code here

    let mut x = 0.000001;

    let replay = vec![
        "0000", "1111", "2222", "3333", "4444", "5555", "6666", "7777", "8888", "9999",
    ];

    let mut file = std::fs::File::create("listing.txt").expect("Creation failure");

    while x < 1.0 {
        x += 0.000001;
        let a = format!("{:.7}", x);
        let o = &a[2..];
        let out = format!("{}{}", reg, o);
        if !out.contains(replay[0])
            && !out.contains(replay[1])
            && !out.contains(replay[2])
            && !out.contains(replay[3])
            && !out.contains(replay[4])
            && !out.contains(replay[5])
            && !out.contains(replay[6])
            && !out.contains(replay[7])
            && !out.contains(replay[8])
            && !out.contains(replay[9])
        {
            writeln!(file, "{} \n", out);
        }
    }


}
