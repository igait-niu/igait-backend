#[macro_export]
macro_rules! print_be{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[Backend] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::blue(&to_print), message);
    }
}
#[macro_export]
macro_rules! print_db{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[DB] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::purple(&to_print), message);
    }
}
#[macro_export]
macro_rules! print_s3{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[S3] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::cyan(&to_print), message);
    }
}
#[macro_export]
macro_rules! print_metis{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[METIS] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::magenta(&to_print), message);
    }
}