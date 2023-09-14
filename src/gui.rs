use iced::widget::{Row, Column, Container, Button,text_input};
use iced::{Element, Length, Sandbox};
use crate::{ytdlp, functions};


#[derive(Default)]
pub struct Styling 
{
    input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message 
{
    InputChanged(String),
    ButtonPressed(ButtonName),
    SubmitPressed,
}

#[derive(Debug, Clone)]
pub enum ButtonName 
{
    Aggiorna,
}

impl Sandbox for Styling 
{
    type Message = Message;

    fn new() -> Self {Styling::default()}

    fn title(&self) -> String {String::from("YT-MP3")}

    fn update(&mut self, message: Message) 
    {
        match message
        {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed(button_name) => 
            {
                match button_name 
                {
                    ButtonName::Aggiorna => ytdlp::install(),
                }
            }
            Message::SubmitPressed => 
            {
                let link=&self.input_value;
                let is_valid = functions::verify_link(link.to_string());
                if is_valid
                {
                    if let Err(err) = ytdlp::download_mp3(&link) 
                    {
                    println!("Errore nel download del video: {}", err);
                    }
                    else 
                    {
                    println!("Download del video completato con successo!");
                    }
                } 
                else 
                {
                    println!("Link non valido!");
                }
            }
        }
    }

    fn view(&self) -> Element<Message> 
    {
        let text_input = text_input("Inserisci il link", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let scarica_button = Button::new("Scarica")
            .on_press(Message::SubmitPressed)
            .padding(10);

        let aggiorna_button = Button::new("Aggiorna")
            .on_press(Message::ButtonPressed(ButtonName::Aggiorna))
            .padding(10);

        let link_input_text_box = Row::new()
            .spacing(20)
            .padding(20)
            .push(text_input)
            .push(scarica_button);

        let bottoni = Row::new()
            .spacing(20)
            .padding(20)
            .push(aggiorna_button);

        let content = Column::new()
            .push(link_input_text_box)
            .push(bottoni);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
