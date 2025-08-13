use chrono::Local;

const RESET: &str = "\x1b[0m";
const COLOR_DEFAULT: &str = "\x1b[0m";
const COLOR_ORANGE: &str = "\x1b[38;5;208m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_PURPLE: &str = "\x1b[35m";

pub fn log(content: &str, color: &str) {
    let now = Local::now().format("%H:%M:%S%.3f");
    println!("{color}{now} {content}{RESET}");
}

pub fn info(content: &str) {
    log(content, COLOR_DEFAULT);
}

pub fn warn(content: &str) {
    log(content, COLOR_ORANGE);
}

pub fn error(content: &str) {
    log(content, COLOR_RED);
}

pub fn debug(content: &str) {
    log(content, COLOR_PURPLE);
}
