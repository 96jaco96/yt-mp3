use std::path::Path;
use regex::Regex;

// Function to check if a file already exists
pub fn file_exists(filename: &str) -> bool 
{
    // Define the path for the file based on the OS
    let path = if cfg!(windows)
    {
        format!(".\\{}.exe", filename)
    } else
    {
        format!("./{}", filename)
    };

    // Convert the path string to a Path
    let path_obj = Path::new(&path);

    path_obj.exists()
}

// Function that uses a regular expression to check if it's a valid YouTube link
pub fn verify_link(link: String) -> bool 
{
    let youtube_regex = Regex::new(r#"(?:https?://)?(?:www\.)?youtube\.com/watch\?v=[\w-]+"#).unwrap();

    if youtube_regex.is_match(&link) 
    {
        return true;
    } 
    else 
    {
        return false;
    }
}
