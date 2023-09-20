use std::fs;
use std::env;



fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args {
        let dir = fs::read_dir(arg);
        let res = match dir {
            Ok(r) => r,
            Err(e) => panic!("Could not read dir {}", e),
        };
        for d in res {
            let dd = match d {
                Ok(t) => t,
                Err(e) => panic!("Could not read dir {}", e),
            };
            let file_name = match dd.file_name().into_string() {
                Ok(r) => r,
                Err(e) => panic!("Could not read file name {:?}", e),
            };
            println!("{}", file_name);
        }

    }

}