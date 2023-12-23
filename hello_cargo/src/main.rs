use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    
    let hello = String::from("नमस्ते");
    println!("{}", hello);

    let mut v = vec![1, 2, 3];

    let third: &i32 = &v[2];
    println!("third: {}", third);
    
    let third: Option<&i32> = v.get(4);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no element."),
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);
    println!("{}", format!("{s2}-{s3}"));

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let greeting_file_result = File::open("hello.txt");

    match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };


}