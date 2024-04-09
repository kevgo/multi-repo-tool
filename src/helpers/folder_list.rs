use big_s::S;
use std::fmt::Write as _;

pub fn render(folders: &[String]) -> String {
  let mut result = S("");
  for (i, folder) in folders.iter().enumerate() {
    match folders.len() {
      10..=99 => writeln!(result, "{:02}. {}", i + 1, folder).unwrap(),
      100..=999 => writeln!(result, "{:03}. {}", i + 1, folder).unwrap(),
      _ => writeln!(result, "{}. {}", i + 1, folder).unwrap(),
    }
  }
  result
}
