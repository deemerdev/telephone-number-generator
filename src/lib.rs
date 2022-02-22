
use rand::Rng;
use std::fmt::format;
use std::fs::File;
use std::io::*;



pub fn gen_rand(a:usize){
    let code = "+0 000 ";//enter country and region code here
    let path = "generated.txt";
    let mut f = File::create(path).expect("err creating");
     for _ in 0..a/2{

        let otp = format!("{:?}",rand::thread_rng().gen_range(0.0..1.0));
        let frmt = &otp[2..9];
        let res = format!("{}{}",code,frmt);

        writeln!(f,"{:?} \n",res).expect("cdnt written");
    }
 
}
