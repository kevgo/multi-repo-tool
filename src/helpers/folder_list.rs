pub fn print(folders: &[String]) {
    for (i, folder) in folders.iter().enumerate() {
        match folders.len() {
            10..=99 => println!("{:02}. {}", i + 1, folder),
            100..=999 => println!("{:03}. {}", i + 1, folder),
            _ => println!("{}. {}", i + 1, folder),
        }
    }
}
