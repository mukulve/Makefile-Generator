use std::fs::File;
use std::io::stdin;
use std::io::Write;
use std::time::Instant;

fn main() {
    //Set variables
    let mut main_file = String::new();
    let mut header_file = String::new();
    let mut c_file = String::new();
    let mut make_file = String::new();
    let gcc_command = "gcc -Wall -std=c99 ";
    let mut temp_str = String::new();
    //Get user input
    println!("Enter Main File ex: main.c : ");
    stdin()
        .read_line(&mut main_file)
        .expect("Couldnt Read From STDIO");
    println!("Enter C Files seperated by spaces ex: 1.c 2.c : ");
    stdin()
        .read_line(&mut c_file)
        .expect("Couldnt Read From STDIO");
    println!("Enter Header File ex: main.h : ");
    stdin()
        .read_line(&mut header_file)
        .expect("Couldnt Read From STDIO");
    //Remove new line character from use input
    let now = Instant::now();
    main_file.truncate(main_file.len() - 1);
    header_file.truncate(header_file.len() - 1);
    c_file.truncate(c_file.len() - 1);
    //Create and Format makefile
    make_file.push_str("output: ");
    for x in c_file.split(" ") {
        temp_str = format!("{}.o ", x.split(".c").next().unwrap_or(""));
        make_file.push_str(&temp_str);
        temp_str.clear();
    }

    temp_str = format!(
        "{}.o\n\t{}",
        main_file.split(".c").next().unwrap_or(""),
        gcc_command
    );
    make_file.push_str(&temp_str);
    temp_str.clear();

    for x in c_file.split(" ") {
        make_file.push_str(x.split(".c").next().unwrap_or(""));
        make_file.push_str(".o ");
    }
    make_file.push_str(&main_file);
    make_file.push_str(" -o output\n");
    for x in c_file.split(" ") {
        temp_str = format!(
            "{}.o: {} {}\n\t{}-c {}\n",
            x.split(".c").next().unwrap_or(""),
            x,
            &header_file,
            gcc_command,
            x
        );

        make_file.push_str(&temp_str);
        temp_str.clear();
    }

    temp_str = format!(
        "{}.o: {} {}\n\t{}-c {}\nclean:\n\trm *.o output",
        main_file.split(".c").next().unwrap_or(""),
        &main_file,
        &header_file,
        gcc_command,
        &main_file
    );
    make_file.push_str(&temp_str);
    temp_str.clear();

    //Show makefile onto console
    println!("----------");
    println!("{}", make_file);
    println!("----------");
    //Create and write makefile
    let mut file = File::create("makefile").expect("Couldnt Create File!");
    writeln!(&mut file, "{}", make_file).unwrap();
    let elapsed = now.elapsed();
    println!("Time Taken : {:.2?}", elapsed);
}
