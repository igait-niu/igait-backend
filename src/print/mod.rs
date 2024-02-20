use colorama::Colored;

// S3 coloring
pub fn print_s3 ( input: &str ) {
    println!("{}", String::from(input).color("blue") );
}

// Database coloring
pub fn print_db ( input: &str ) {
    println!("{}", String::from(input).color("bright magenta") );
}

// Backend coloring
pub fn print_be ( input: &str ) {
    println!("{}", String::from(input).color("green") );
}