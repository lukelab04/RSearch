#![allow(non_snake_case)]

use std::{env};
use std::fs;
mod cmdin;

#[derive(Debug)]
struct SearchCriteria {
    name: String,
    path: String,
    debug: bool,
}

fn GetCommandArgs() -> SearchCriteria {

    let path = match env::current_dir() {
        Ok(val) => val,
        Err(val) => panic!("Error parsing input {}.", val),
    };

    let arguments = cmdin::CommandParser::GetInput();
    if arguments.len() < 1 {panic!("Too few arguments.");}
    let mut sc = SearchCriteria {name: String::from(""), path: String::from(path.into_os_string().into_string().expect("Could not parse input.")), debug: false};

    for arg in arguments {
        match arg.flag {
            Some(val) => {
                match val.as_str() {
                    "-p" => {sc.path = String::from(arg.val);}
                    "-d" => {sc.debug = true;}
                    _ => {panic!("Unknown flag {}", &val);}
                }
            },

            None => {
                sc.name = String::from(arg.val);
            }
        }
    }

    return sc;

}

fn SearchDirRecursive(output : &mut Vec<String>, pathstr : &str, searchTerm : &str, filesSearched: &mut i32) {
    *filesSearched+=1;
    let paths = fs::read_dir(pathstr);
    //println!("{}", pathstr);
    match paths {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(path) => {
                        let name = path.file_name();                
                        let mut tmp = String::from(pathstr).to_owned();
                        if tmp.chars().last().unwrap() != '/' {tmp.push_str("/");}

                        let mut okname : &str = "";

                        match name.to_str() {
                            Some(s) => {okname = s;}
                            _ => {}
                        }

                        tmp.push_str(okname);
                        
                        SearchDirRecursive(output, tmp.as_str(), searchTerm, filesSearched);
                        if String::from(okname).contains(searchTerm) {output.push(tmp)}
                    },
                    Err(_e) => {}
                }
            }
        },
        Err(_e) => {}
    }
}

fn SearchForFiles(sc : SearchCriteria) -> Vec<String> {
    let mut output : Vec<String> = vec![];

    if sc.debug {println!("{:?}", &sc);}
    let mut fs: i32 = 0;
    SearchDirRecursive(&mut output, sc.path.as_str(), sc.name.as_str(), &mut fs);

    if sc.debug {println!("Searched {} files", fs)}

    return output;
}

fn main() {
    let sc = GetCommandArgs();
    let results = SearchForFiles(sc);
    for elem in results {
        println!("{}", &elem);
    }
}
