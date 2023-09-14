use iced::Sandbox;
mod gui;
mod functions;
mod ytdlp;


fn main() 
{
    // Check and install yt-dlp if needed
    if functions::file_exists("yt-dlp") 
    {
        println!("yt-dlp gia' presente!");
    }
    else 
    {
        ytdlp::install();    
    }
    // Start the GUI
    let _ = gui::Styling::run(Default::default());
}
