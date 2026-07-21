use walkdir::{DirEntry, WalkDir};

pub fn dir_walk(root: String, file_extension: String) -> Vec<String> {
    let mut result = vec![];

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| is_valid(e, &file_extension))
    {
        //let entry = entry.unwrap();
        //println!("{}", entry.path().display());
        result.push(entry.path().display().to_string());
    }

    result
}

fn is_valid(entry: &DirEntry, file_extension: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(file_extension))
        .unwrap_or(true)
}
