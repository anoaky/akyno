pub fn main() {}

fn usage() -> ! {
    println!("Usage: toycc <pass> <file>");
    println!("Available passes: -lexer, -parser");
    std::process::exit(-1);
}
