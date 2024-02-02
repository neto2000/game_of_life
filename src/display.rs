use termion::{self};
use iced::{Element, Sandbox, Settings};
use iced::widget::{button,column,text,Column};



pub fn gui() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter{
    value: i32,   
}
#[derive(Debug,Clone, Copy)]
pub enum Message {
    Increment,
}

impl Sandbox for Counter {


    type Message = Message;

    fn new() -> Self {
        Self{ value: 0 }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Message) {
        // This application has no interactions
        //
        match message {
            Message::Increment => {
                self.value += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::Increment),

            text(self.value).size(50),
        ].into()
    }
}



pub fn write_block(x: u16, y: u16) {
    print!("{}{}", termion::cursor::Goto(x + 1, y + 1), termion::color::Bg(termion::color::White));

    print!(" ");

    print!("{}", termion::color::Bg(termion::color::Reset));
}

pub fn delete_block(x: u16, y: u16) {

    print!("{}", termion::cursor::Goto(x + 1, y + 1));

    print!(" ");
}
