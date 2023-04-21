use std::env;
use std::path::Path;
use regex::{Regex,Captures};
        
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input_path = Path::new(&args[1]);
    let input_text = String::from_utf8(std::fs::read(&input_path).unwrap()).unwrap();
    let re = Regex::new(r"\$ENV\{(\w+)\}").unwrap();
    let result = re.replace_all(&input_text, |caps: &Captures| {
        assert_eq!(caps.len(), 2);
        match std::env::var(&caps[1]) {
            Ok(value) => value,
            Err(err) => match err {
                env::VarError::NotPresent => panic!("Env var: {} not present", &caps[1]),
                env::VarError::NotUnicode(_) => panic!("Env var: {} must be in unicode", &caps[1]),
            },
        }
    });
    println!("{}", result);
}