use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    //Set variables
    let mut main_file = String::new();
    let mut header_file = String::new();
    let mut c_file = String::new();
    let mut make_file = String::new();
    let gcc_command = "gcc -Wall -std=c99 ";
    //Get user input
    println!("Enter Main File ex: main.c : ");
    io::stdin()
        .read_line(&mut main_file)
        .expect("Couldnt Read From STDIO");
    println!("Enter C Files seperated by spaces ex: 1.c 2.c : ");
    io::stdin()
        .read_line(&mut c_file)
        .expect("Couldnt Read From STDIO");
    println!("Enter Header File ex: main.h : ");
    io::stdin()
        .read_line(&mut header_file)
        .expect("Couldnt Read From STDIO");
    //Remove new line character from use input
    main_file.truncate(main_file.len() - 1);
    header_file.truncate(header_file.len() - 1);
    c_file.truncate(c_file.len() - 1);
    //Create and Format makefile
    make_file.push_str("output: ");
    for x in c_file.split(" ") {
        make_file.push_str(x.split(".c").next().unwrap_or(""));
        make_file.push_str(".o ");
    }
    make_file.push_str(main_file.split(".c").next().unwrap_or(""));
    make_file.push_str(".o\n\t");
    make_file.push_str(gcc_command);
    for x in c_file.split(" ") {
        make_file.push_str(x.split(".c").next().unwrap_or(""));
        make_file.push_str(".o ");
    }
    make_file.push_str(&main_file);
    make_file.push_str(" -o output\n");
    for x in c_file.split(" ") {
        make_file.push_str(x.split(".c").next().unwrap_or(""));
        make_file.push_str(".o: ");
        make_file.push_str(x);
        make_file.push_str(" ");
        make_file.push_str(&header_file);
        make_file.push_str("\n\t");
        make_file.push_str(gcc_command);
        make_file.push_str("-c ");
        make_file.push_str(x);
        make_file.push('\n');
    }
    make_file.push_str(main_file.split(".c").next().unwrap_or(""));
    make_file.push_str(".o: ");
    make_file.push_str(&main_file);
    make_file.push_str(" ");
    make_file.push_str(&header_file);
    make_file.push_str("\n\t");
    make_file.push_str(gcc_command);
    make_file.push_str("-c ");
    make_file.push_str(&main_file);
    make_file.push('\n');
    make_file.push_str("clean:\n\trm *.o output");
    //Show makefile onto console
    println!("----------");
    println!("{}", make_file);
    println!("----------");
    //Create and write makefile
    let mut file = File::create("makefile").expect("Couldnt Create File!");
    writeln!(&mut file, "{}", make_file).unwrap();
}
