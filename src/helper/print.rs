/// Prints indication of a backend-related message.
/// 
/// # Example
/// ```rust
/// print_be!(0, "Unusual status code detected, purging accordingly: {code:#?}");
/// ```
/// 
/// # Potential Reasons for Failure
/// * The `task_number` is not a valid number.
/// * The `message` is not a valid string.
#[macro_export]
macro_rules! print_be{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[Backend] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::blue(&to_print), message);
    }
}

/// Prints indication of a database-related message.
/// 
/// # Example
/// ```rust
/// print_db!(0, "Unusual status code detected, purging accordingly: {code:#?}");
/// ```
/// 
/// # Potential Reasons for Failure
/// * The `task_number` is not a valid number.
/// * The `message` is not a valid string.
#[macro_export]
macro_rules! print_db{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[DB] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::purple(&to_print), message);
    }
}

/// Prints indication of a S3-related message.
/// 
/// # Example
/// ```rust
/// print_s3!(0, "Unusual status code detected, purging accordingly: {code:#?}");
/// ```
/// 
/// # Potential Reasons for Failure
/// * The `task_number` is not a valid number.
/// * The `message` is not a valid string.
#[macro_export]
macro_rules! print_s3{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[S3] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::cyan(&to_print), message);
    }
}

/// Prints indication of a Metis-related message.
/// 
/// # Example
/// ```rust
/// print_metis!(0, "Unusual status code detected, purging accordingly: {code:#?}");
/// ```
/// 
/// # Potential Reasons for Failure
/// * The `task_number` is not a valid number.
/// * The `message` is not a valid string.
#[macro_export]
macro_rules! print_metis{
    ($a:expr, $b:expr) => {
        let message = format!($b);
        let to_print = format!("[Metis] [Task {}]", $a);

        println!("{} {}", <&str as colored::Colorize>::magenta(&to_print), message);
    }
}