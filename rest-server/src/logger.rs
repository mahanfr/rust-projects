use colored::Colorize;
use crate::DEV;

#[allow(dead_code)]
pub fn warning(str: &str) {
    println!("{} {:?}"," Warning ".on_yellow().bold(),str)
}

#[allow(dead_code)]
pub fn info(str: &str) {
    println!("{} {:?}"," Info ".on_bright_cyan().bold(),str)
}

#[allow(dead_code)]
pub fn error(str: &str) {
    println!("{} {:?}"," Error ".on_bright_red().white().bold(),str)
}

#[allow(dead_code)]
pub fn debug(str: &str){
    if DEV == true{
        println!("{:?}",str)
    }
}