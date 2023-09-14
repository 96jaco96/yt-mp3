use reqwest;
use std::fs::File;
use std::io;
use std::process::{Command, ExitStatus};


// Function to set the yt-dlp download url depending on the OS
fn set_url () -> String
{
    // Change the url accordingly if running on Windows or Linux
    let url: String = if cfg!(windows)
    {
        String::from("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe")
    } 
    else 
    {
        String::from("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp")
    };
    return url;   
}

// Function to download a file from a given URL and save it to a static path
pub fn update(url: &str) -> Result<(), Box<dyn std::error::Error>>
{
    // Create a reqwest Client
    let client = reqwest::blocking::Client::new();

    // Send a GET request to the URL
    let response = client.get(url).send()?;

    // Check if the request was successful (status code 200)
    if !response.status().is_success() 
    {
        return Err(format!("Download fallito con codice: {}", response.status()).into());
    }

    // Set the save file path for the downloaded file on windows or Linux
    let path:&str = if cfg!(windows)
    {
        "yt-dlp.exe"
    } 
    else
    {
        "yt-dlp"
    };

    // Create or open a file at the save_path
    let mut file = File::create(path)?;

    // Copy the response body to the file
    io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;

    println!("Yt-dlp scaricato correttamente!");

    // Add +x permission if running on Linux
    if !cfg!(windows)
    {
        let output = Command::new("chmod")
        .arg("+x")
        .arg("./yt-dlp")
        .output()
        .expect("Failed to execute command");

        if output.status.success() 
        {
            println!("Permesso di esecuzione aggiunto correttamente!");
        } 
        else 
        {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            println!("Errore nell'aggiunta del permesso di esecuzione: {}", error_msg);
        }
    }

    Ok(())
}

pub fn install()
{
    let url = &set_url();
    if let Err(err) = update(url) 
    {
    println!("Errore di installazione: {}", err);
    }
    else 
    {
    println!("Yt-dlp installato e aggiornato con successo!");
    }
}

pub fn download_mp3(link: &str) -> Result<ExitStatus, std::io::Error> 
{
    if cfg!(windows)
    {
    let status = Command::new(".\\yt-dlp.exe")
        .args(&["--extract-audio", "--audio-format", "mp3", link])
        .status()?;
        Ok(status)
    }
    else 
    {
    let command_str = format!("./yt-dlp --extract-audio --audio-format mp3 {}", link);
    
    let status = Command::new("sh")
        .arg("-c")
        .arg(&command_str)
        .status()?;
        Ok(status)
    }
}
