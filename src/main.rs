use std::env;
use std::path;
use std::fs;
use regex::Regex;

fn process_file(file_path: &path::Path, file_str: String) -> String{
    let pattern = Regex::new(r"\{/(.*)/\}").unwrap();
    let mut out_file = file_str.clone();
    for cap in pattern.captures_iter(&file_str){
        // println!("{}", &cap[0]);
        let mut replace_with_file_location = cap[1].trim();
        let mut replace_with_file_path = file_path.clone();
        while replace_with_file_location.starts_with(".."){
            replace_with_file_location = &replace_with_file_location[3..];
            replace_with_file_path = replace_with_file_path.parent().unwrap();
        }
        let mut replace_with_file = replace_with_file_path.parent().unwrap().to_str().unwrap().to_owned();
        replace_with_file.push_str(&"/");
        replace_with_file.push_str(replace_with_file_location);
        replace_with_file.push_str(&".md");
        // println!("{}", replace_with_file);
        let replace_file = process_file(path::Path::new(&replace_with_file), fs::read_to_string(&replace_with_file).unwrap());
        out_file = out_file.replace(&cap[0], &replace_file);
    }
    out_file
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let pattern = Regex::new(r"\{/(.*)/\}").unwrap();
    for arg in &args[1..]{
        let file_path = path::Path::new(arg);
        if !file_path.exists() {continue;}
        let file = fs::read_to_string(arg).unwrap();
        let out_file = process_file(file_path, file);
        println!("{}", out_file);
    }
    //println!("{:?}", args)
}
