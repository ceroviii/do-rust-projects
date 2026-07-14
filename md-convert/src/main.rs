use std::fs;

mod inline;
mod parser;
mod renderer;

static USAGE: &str = r#"
- md-convert input.md output.html
- md-convert input.md               ← defaults to input.html
"#;

static RESET: &str = "\x1b[0m";

fn main() {
    /*let re = Regex::new(r"\*([\w\d\s^*]+)\*").unwrap();
    println!(
        "> > {:?}",
        re.captures("*this is italic* **this is bold**").unwrap()
    );*/

    let some_args = std::env::args().collect::<Vec<_>>();
    let mut src_file = String::new();
    let mut dest_file = String::new();
    if some_args.len() == 2 {
        src_file = some_args[1].clone();
        let src_file_name_split = &src_file.split(".md").collect::<Vec<_>>();
        dest_file = src_file_name_split[0].to_owned() + ".html";
    } else if some_args.len() == 3 {
        src_file = some_args[1].clone();
        dest_file = some_args[2].clone();
    } else {
        println!("\x1b[31mUsage{}", RESET);
        println!("{}", USAGE);
        return;
    }

    let mut rendered_result = String::new();
    // process content from src_file
    if let Ok(src_content) = fs::read_to_string(&src_file) {
        rendered_result = renderer::render(src_content, src_file);
    } else {
        println!("\x1b31m(x) Error occured while reading file!");
        return;
    }

    // save processed content to dest_file
    fs::write(dest_file, rendered_result).expect("\x1b[31m(Err): File Not Found");
    //println!("{} -> {}", src_file, dest_file)
}
