use termion::{color, style};

fn main() {
    println!("Hello, world!");
    
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    //println!("{}Hello, world!{}", color::Fg(color::Green), style::Reset);
    let termsize = termion::terminal_size().unwrap();
    println!("termsize {} {}", termsize.0, termsize.1);
    
    // put the top on the tree
    println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 -2, termsize.1 / 2 -4), color::Fg(color::Red), "\\|/");
    println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 -2, termsize.1 / 2 -3), color::Fg(color::Red), "-★-");
    println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 -1, termsize.1 / 2 -2), color::Fg(color::Yellow), "★");
    println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 -1, termsize.1 / 2 -1), color::Fg(color::Yellow), "★");
    println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 -1, termsize.1 / 2 ), color::Fg(color::Yellow), "★");
    
    
    for i in 0..termsize.1 / 2 {
        let mut tree = String::new();
        // green tree
        for _j in 1..i * 2
        {
            tree.push_str("X");
        }
        println!("{}{}{}", termion::cursor::Goto(termsize.0 /2 - i, termsize.1 / 2 + i), color::Fg(color::Green), tree);

        // red balls
        for j in 1..i * 2 
        {
            
            if j % 4 == 0 {
                println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 - j + i , termsize.1 / 2 + i), color::Fg(color::Red), "●");
            }
        }
        // candles
        if i %4 == 1 {
            println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 - i -2, termsize.1 / 2 + i), color::Fg(color::Yellow), "⅄");
            println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 + i , termsize.1 / 2 + i), color::Fg(color::Yellow), "⅄");
        }
        if i %4 == 2 {
            println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 - i -1, termsize.1 / 2 + i), color::Fg(color::Red), "‖");
            println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 + i -1, termsize.1 / 2 + i), color::Fg(color::Red), "‖");
        }
    }
    print!("{}", style::Reset);

}
