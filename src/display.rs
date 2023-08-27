use termion::{self};

pub fn write_block(x: u16, y: u16) {
    print!("{}{}", termion::cursor::Goto(x + 1, y + 1), termion::color::Bg(termion::color::White));

    print!(" ");

    print!("{}", termion::color::Bg(termion::color::Reset));
}

pub fn delete_block(x: u16, y: u16) {

    print!("{}", termion::cursor::Goto(x + 1, y + 1));

    print!(" ");
}
