#[macro_export]
macro_rules! warning {
    ($val:expr $(, $opt:expr)*) => {
        print!("{} ","[warning]".bold().bright_yellow());
        println!($val,$($opt),*);
    };
}
#[macro_export]
macro_rules! info {
    ($val:expr $(, $opt:expr)*) => {
        print!("{} ","[info]".bold().bright_blue());
        println!($val,$($opt),*);
    };
}
