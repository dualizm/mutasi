/**********************************************************
*  PROJECT_NAME: Mutasi;
*  VERSION: 0.1.0;
*  AUTHOR: Notidman;
***********************************************************/
use std::env;
use console::style;
include!("game.rs");

fn main()
{  
    for arg in env::args().skip(1) {
        let arg: &str = arg.as_str();
        match arg 
        {
            "--version" | "-v" => println!("{}{}", "Mutasi version", style(" 1.0").green().bold()),

            "-h" | "--help" =>  println!("\n{}{}\n\n{}{}\n\n{}{}\n",
                style("--help -h").red(),    " -> Show help commands.",
                style("--version -v").red(), " -> Show the version of the program.",
                style("--start -s").red(),   " -> Starts the game with the multiplication table."),

            "-start" | "-s" => game(),

            _ => print!("Mutasi: {} {}", style("error:").red().bold(), "Not correct args!")
        };
    }
}