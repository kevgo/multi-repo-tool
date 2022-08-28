pub fn walk(current_dir: Utf8PathBuf) -> Vec<Step> {
    let mut result = vec![];
    let dirs = get_subdirs(&current_dir)?;
    let mut count = 1;
    for dir in dirs {
        result.push(operations::chdir(count, dir));
        count += 1;
        result.push(operations::exit(count));
        count += 1;
    }
    result.push(operations::chdir(count, current_dir.into_string()));
    result
}
