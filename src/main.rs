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
    
    
    for i in 0..termsize.1 / 2 + 1 {
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
    println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 - termsize.1 / 2 - 5, termsize.1 -1 ), color::Fg(color::Red), "🦀");
    println!("{}{}{}", termion::cursor::Goto(termsize.0 / 2 - termsize.1 / 2 - 6, termsize.1 -2 ), color::Fg(color::White), "\\");
    println!("{}{}M{}e{}r{}r{}y {}C{}h{}r{}i{}s{}t{}m{}a{}s{}!", termion::cursor::Goto(termsize.0 / 2 - termsize.1 / 2 - 22, termsize.1 -3 ), 
    color::Fg(color::Rgb(255, 0,0)),
    color::Fg(color::Rgb(255, 148,11)),
    color::Fg(color::Rgb(255, 255,0)),
    color::Fg(color::Rgb(1, 204,0)),
    color::Fg(color::Rgb(3, 192,198)),
    color::Fg(color::Rgb(0, 0, 254)),
    color::Fg(color::Rgb(118, 44, 167)),
    color::Fg(color::Rgb(254, 152,191)),
    color::Fg(color::Rgb(255, 0,0)),
    color::Fg(color::Rgb(255, 148,11)),
    color::Fg(color::Rgb(255, 255,0)),
    color::Fg(color::Rgb(1, 204,0)),
    color::Fg(color::Rgb(3, 192,198)),
    color::Fg(color::Rgb(0, 0, 254)),
    color::Fg(color::Rgb(118, 44, 167)));
    print!("{}", style::Reset);

}
