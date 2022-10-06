pub fn render(folders: &[String]) -> String {
    let mut result = String::new();
    for (i, folder) in folders.iter().enumerate() {
        match folders.len() {
            10..=99 => result.push_str(&format!("{:02}. {}\n", i + 1, folder)),
            100..=999 => result.push_str(&format!("{:03}. {}\n", i + 1, folder)),
            _ => result.push_str(&format!("{}. {}\n", i + 1, folder)),
        }
    }
    result
}
