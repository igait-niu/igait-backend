#[macro_export]
macro_rules! print_be{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[Backend] [Task {}]", $a);

        println!("{} {}", to_print.blue(), message);
    }
}
#[macro_export]
macro_rules! print_db{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[DB] [Task {}]", $a);

        println!("{} {}", to_print.purple(), message);
    }
}
#[macro_export]
macro_rules! print_s3{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[S3] [Task {}]", $a);

        println!("{} {}", to_print.cyan(), message);
    }
}
#[macro_export]
macro_rules! print_metis{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[METIS] [Task {}]", $a);

        println!("{} {}", to_print.magenta(), message);
    }
}