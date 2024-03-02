pub(crate) fn options() {
    let file_content = read_file();
    
    if let Some(content) = file_content {
        println!("{}", content)
    }
}

fn read_file() -> Option<String> {
    Some(String::from("Some content"))
    // None
}