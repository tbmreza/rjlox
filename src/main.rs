use scanner::Scanner;
use std::io::Read;
mod scanner;

fn main() {
    let mut _had_error = false;

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        // Wrong usage
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        // Run script
        run_file(&args[1], &mut _had_error);
    } else {
        // Open REPL
        run_prompt(&mut _had_error);
    }
}

fn run_file(path: &str, mut _had_error: &mut bool) {
    let mut f = std::fs::File::open(&path).expect("File not found.");
    let metadata = std::fs::metadata(&path).expect("Metadata not found.");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow.");

    let s = String::from_utf8(buffer).expect("Invalid utf8.");
    run(&s, &mut _had_error);
    if *_had_error {
        std::process::exit(65);
    }
}

fn run_prompt(mut _had_error: &mut bool) {
    // instantiate Scanner

    println!("rlox - Following along Crafting Interpreters book. reza lox :)");
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("stdin error.");
        // run(&mut _had_error)
        print!("{}", &input);
        *_had_error = false;
    }
}

fn run(source_code: &str, mut _had_error: &mut bool) {
    // instantiate Scanner
    let mut _scanner = Scanner::new(source_code, &mut _had_error);
    _scanner.scan_tokens();
    for token in &_scanner.tokens {
        print!("{}", token.lexeme);
    }
}

fn report(line: u32, which: &str, message: &str, mut _had_error: &mut bool) {
    print!("[line {}] Error{}: {}", line, which, message);
    *_had_error = true;
}
fn error(line: u32, message: &str, mut _had_error: &mut bool) {
    report(line, "", message, &mut _had_error);
}
