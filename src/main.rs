use termion::{color, style};

fn main() {
    println!("Hello, world!");
    
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    //println!("{}Hello, world!{}", color::Fg(color::Green), style::Reset);
    let termsize = termion::terminal_size().unwrap();
    println!("termsize {} {}", termsize.0, termsize.1);
    // println!("{}{}{}", termion::cursor::Goto(termsize.0 /2, termsize.1 / 2), color::Fg(color::Green), "X");
    // println!("{}{}{}", termion::cursor::Goto(termsize.0 /2 - 1, termsize.1 / 2 + 1), color::Fg(color::Green), "YYY");
    // println!("{}{}{}", termion::cursor::Goto(termsize.0 /2 - 2, termsize.1 / 2 + 2), color::Fg(color::Green), "ZZZZZ");
    // println!("{}{}{}", termion::cursor::Goto(termsize.0 /2 - 3, termsize.1 / 2 + 3), color::Fg(color::Green), "1111111");
    let x_top = termsize.0 / 2;
    for i in 0..termsize.1 - 2 {
        let mut tree = String::new();
        for j in 1..i * 2
        {
            tree.push_str("X");
        }
        println!("{}{}{}", termion::cursor::Goto(termsize.0 /2 - i, termsize.1 / 2 + i), color::Fg(color::Green), tree);

    }
    print!("{}", style::Reset);

}
