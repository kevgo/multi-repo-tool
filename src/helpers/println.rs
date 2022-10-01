macro_rules! println_bold {
    ($a:expr,$b:expr) => {
        let println_bold_text = format!($a, $b);
        println!("{}", println_bold_text.bold());
    };
}

pub(crate) use println_bold;
