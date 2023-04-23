#![allow(non_snake_case)]
use colored::*;
use std::{fs, env};
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::path::Path;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let files = "example2.csv".to_string();
        let contents=fs::read_to_string(files).expect("Should have been able to read the file");
        remove_postfix(&contents);
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let contents=fs::read_to_string(file).expect("Should have been able to read the file");
    remove_postfix(&contents)
}

#[allow(unused_assignments)]
fn remove_postfix(contents: &String){ 
    
    let mut line_vec: Vec<String> = Vec::new();

    println!("Deleted postfix:");
    
    let re = Regex::new(r"\([a-zA-Z]\)").unwrap();
    
    line_vec = contents.lines().filter(|x| re.is_match(x)).map(|x| x.to_string()).collect();

    find_low_let(line_vec);
    
    let re = Regex::new(r"\s\([a-zA-Z]\)").unwrap();
    let mut contents = re.replace_all(&contents, "").to_string();
    let re = Regex::new(r"\([a-zA-Z]\) ").unwrap();
    contents = re.replace_all(&contents, "").to_string();
    contents = contents.lines().map(|x| x.trim_end_matches(',').to_string()).collect::<Vec<String>>().join("\n");
    sort_and_save(contents);
}

fn sort_and_save(contents: String)
{
    let mut name_of_file = String::new();
    
    println!("\nwrite name of the final file:");
    io::stdin()
        .read_line(&mut name_of_file)
        .expect("Failed to read line");
    let mut fin_name = name_of_file.trim().to_string();
    let mut file_num=1;

    while Path::new(&(fin_name.clone() + ".csv")).exists()
    {
        fin_name = name_of_file.trim().to_string() + &file_num.to_string();
        file_num+=1;
    }

    let mut buffer = File::create(fin_name + ".csv").unwrap();
    let mut splits_vec: Vec<String> = Vec::new();
    let mut first = true;
    let mut first_line= "";

    for line in contents.lines()
    {
        if first
        {
            first_line=line;
            first=false;
            continue;
        }
        splits_vec.push(line.to_string());
    }
    splits_vec.sort();
    splits_vec.push(first_line.to_string());
    splits_vec.rotate_right(1);
    
    for line in splits_vec.iter()
    {
        buffer.write_all((line.to_string()+"\n").as_bytes()).unwrap();
    }
}


fn find_low_let(line_vec: Vec<String>) {
    let re = Regex::new(r"\([a-z]\)").unwrap();
    for line in line_vec.iter()
    {
        let splits: Vec<&str> = line.split("\",").collect();
        let name_with_postfix: String = splits[0].trim_matches('"').to_string();
        if re.is_match(&name_with_postfix)
        {
            postf_select_low(&name_with_postfix);
            println!("{}", "The prefix is written with a small letter!".to_string().red());
        }
        let re =Regex::new(r"\([A-Z]\)").unwrap();
        if re.is_match(&name_with_postfix)
        {
            postf_select_big(&name_with_postfix);
        }
    } 
}

fn postf_select_low(line: &String)
{
    let re = Regex::new(r"\([a-z]\)").unwrap();
    for words in line.split_whitespace()
    {
        if re.is_match(&words)
        {
            print!("{}", words.red());
            continue;
        }
        print!("{words} ");
    }
    println!();
}

fn postf_select_big(line: &String){
    let re = Regex::new(r"\([A-Z]\)").unwrap();
    for words in line.split_whitespace()
    {
        if re.is_match(&words)
        {
            print!("{}", words.blue());
            continue;
        }
        print!("{words} ");
    }
    println!();
}