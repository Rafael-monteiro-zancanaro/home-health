use colored::Colorize;

const INFO_ICON: &str = "🛈";
const WARNING_ICON: &str = "⚠";
const ERROR_ICON: &str = "✖";

pub fn info(message: &str) {
    println!("{}", format!("{} INFO: {} [at {}]", INFO_ICON, message, timestamp()).blue());
}

pub fn warning(message: &str) {
    println!("{}", format!("{} WARNING: {} [at {}]", WARNING_ICON, message, timestamp()).yellow());
}

pub fn error(message: &str) {
    println!("{}", format!("{} ERROR: {} [at {}]", ERROR_ICON, message, timestamp()).red());
}


fn timestamp() -> String {
    return chrono::offset::Local::now().format("%d/%m/%Y %H:%M").to_string();
}
