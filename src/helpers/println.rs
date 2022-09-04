macro_rules! println_bold {
    ($a:expr,$b:expr) => {
        let println_bold_text = format!($a, $b);
        println!("{}", println_bold_text.bold());
    };
}

macro_rules! println_error {
    ($a:expr,$b:expr) => {
        print!("{}", "ERROR: ".red().bold());
        let println_error_text = format!($a, $b);
        println!("{}", println_error_text.red());
    };
}

pub(crate) use println_bold;
pub(crate) use println_error;
