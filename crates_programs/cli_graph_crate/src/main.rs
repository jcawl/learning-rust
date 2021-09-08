use std::env;
use std::io::{self};
use std::process::{Command, Stdio};
use std::collections::HashMap;
use piechart::{Chart, Color, Data};
use fastrand::u8;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let mut path = "./";
    let mut recursive_flag = false;

    if args.len() == 2 {
        path = &args[1];
    } else if args.len() == 3 {
        path = &args[1];
        if args[2] == "-R" {
            recursive_flag = true;
        } else {
            recursive_flag = false;
        }
    }

    let mut ls;

    if recursive_flag {
        //recursively list files and directories in current directory
        ls = Command::new("ls")
            .arg("-R")
            .arg("-p")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");
    } else {
        //no recursion
        ls = Command::new("ls")
            .arg("-p")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");
    }

    //pipe output to get only files
    let mut grep = Command::new("grep")
        .arg("-v")
        .arg("/")
        .stdin(ls.stdout.take().unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    //pipe output to get only files with extensions
    let mut grep2 = Command::new("grep")
        .arg(r".*\.")
        .stdin(grep.stdout.take().unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let sed = Command::new("sed")
    .arg(r"s/.*\.//")
    .stdin(grep2.stdout.take().unwrap())
    .output()
    .expect("Failed to execute command");

    let output_string = String::from_utf8( sed.stdout).unwrap();
    // let output_vec = output_string.split("\n").collect::<Vec<&str>>();
    let mut frequency_count: HashMap<&str, usize> = HashMap::new();

    //sperate string using delimeter
    for ext in output_string.split("\n") {
        let is_valid = ext.chars().all(char::is_alphanumeric);
        if is_valid && ext.len() > 0 {
            //when encountering a new word or_insert will set the value to 0
            //it returns a mutable reference to that value
            let count = frequency_count.entry(ext).or_insert(0);
            //we dereference it so we can assign a new value
            *count += 1;
        }
    }


    if frequency_count.len() > 0 {
        //init pie Chart
        let mut data = Vec::new();

        for (ex, num) in &frequency_count {
            data.push(Data {label: ex.to_string(), value: *num as f32, color: Some(Color::RGB(get_rand(),get_rand(),get_rand()).into()), fill: '×'});
        }

        Chart::new()
            .radius(9)
            .aspect_ratio(3)
            .legend(true)
            .draw(&data);
    } else {
        println!("No files found");
    }

    Ok(())
}

fn get_rand() -> u8 {
    u8(0..255)
}
