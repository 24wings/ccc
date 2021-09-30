use std::thread;
use std::time::Duration;

use console::style;
use console::Term;
use console::Emoji;
use indicatif::ProgressBar;
use dialoguer::Confirm;
use dialoguer::MultiSelect;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Sort;

pub mod cc {
    pub fn say_hello() {
        println!("hello");
    }
}

fn main() {
    let current_dir= std::env::current_dir().expect("err");
    println!("{}",current_dir.to_str().expect(""));

    let template: &str = include_str!("../templates/index.html");
    println!("{}", template);

    let term = Term::stdout();
    term.write_line("Hello World!")
        .expect("error in write line");
    thread::sleep(Duration::from_millis(2000));
    term.clear_line().expect("error in clear line");
    term.clear_line().expect("error in clear line");
    term.clear_line().expect("error in clear line");
    term.clear_screen().expect("error in clear line");
    term.set_title("心流代码");

    
    

    println!("This is {} neat", style("quite").cyan());
    println!("[3/4] {}Downloading ...", Emoji("🚚 ", ""));
println!("[4/4] {} Done!", Emoji("✨", ":-)"));
    term.hide_cursor();
    if Confirm::new().with_prompt("Do you want to continue?").interact().expect("a") {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }
    let items = vec![
        "Option 1", "Option 2","Option 3", "Option 4","Option 5", "Option 6","Option 7", "Option 8","Option 9", "Option 10",
        "Option 11", "Option 12","Option 13", "Option 14","Option 15", "Option 16","Option 17", "Option 18","Option 19", "Option 20"
        
    ];
    let items2 = vec![
        true,false
    ];

    let items_to_order = vec!["Item 1", "Item 2", "Item 3"];
let ordered = Sort::new()
    .with_prompt("Order the items")
    .items(&items_to_order)
    .interact().expect("A");
    
    
let chosen : Vec<usize> = MultiSelect::new()
    .items(&items)
    .paged(true)
    .defaults(&items2)
    // .with_theme(ColorfulTheme.default())
    .interact().expect("a");
    let bar = ProgressBar::new(1000);
    for _ in 0..1000 {
        thread::sleep(Duration::from_millis(20));
        bar.inc(1);
        // ...
    }
    bar.finish();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error in read");
    
    thread::sleep(Duration::from_millis(2000));
   
}
