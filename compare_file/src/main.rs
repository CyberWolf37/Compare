use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;

fn main() {

    let file_l = String::from(std::env::args().nth(1).expect("no path origin"));
    let file_r = String::from(std::env::args().nth(2).expect("no path Compare"));
    
    let file_l = read_file(&file_l);
    let file_r = read_file(&file_r);

    compare(&file_l, &file_r);
 
}

fn read_file(file: &String) -> String {

    // Create a path to the desired file
    let path = Path::new(file);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s,
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed

}

fn file_make_Tab(file: &String) -> BTreeMap<&str,&str> {

    let mut map = BTreeMap::new();

    //lines.next();
    //lines.next();

    //let line = lines.next();

    let words_in_lines = words_by_line(file.as_str());

    for line in words_in_lines {
        
        if line.len() >= 2 {
            map.insert(line[0], line[1]);
        } 
    }

    map.remove("Nom");
    map.remove("Groupe");

    map
}

fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

fn compare<'a,'b>(l: &'a String,r: &'b String) {

    let left_file = file_make_Tab(l);
    let right_file = file_make_Tab(r);

    for (key,value) in left_file.iter() {
        for (keyJ,valueJ) in right_file.iter() {
            if key == keyJ {
                if value != valueJ {
                    println!("Origine: {} , Repere: {}  //  Compare: {} , Repere: {}",key,value,keyJ,valueJ);
                }
            }
        }        
    }
}
