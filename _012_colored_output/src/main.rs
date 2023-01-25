use termion::{color, style};

fn main() {
    println!("{}Magenta, {}", color::Fg(color::LightMagenta), String::from("test"));
    println!("{}Blue", color::Bg(color::LightWhite));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    println!("{}Just plain italic{}", style::Italic, style::Reset);
}
