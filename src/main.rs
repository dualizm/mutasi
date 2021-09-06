/**********************************************************
*  PROJECT_NAME: Mutasi;
*  VERSION: 0.1.5;
*  AUTHOR: Notidman;
***********************************************************/
use std::env;
use console::style;
include!("game.rs");
include!("table.rs");

fn main()
{  
    for arg in env::args().skip(1) {
        let arg: &str = arg.as_str();
        match arg 
        {
            "--version" | "-v" => println!("{}{}", "Mutasi version", style(" 1.5").green().bold()),

            "--table" | "-t" => show_table(),

            "--help" | "-h"  =>  println!("\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n",
                style("--help -h").red(),    " -> Show help commands.",
                style("--version -v").red(), " -> Show the version of the program.",
                style("--table -t").red(),   " -> Shows the multiplication table.",
                style("--start -s").red(),   " -> Starts the game with the multiplication table."),

            "--start" | "-s" => game(),

            _ => print!("Mutasi: {} {}", style("error:").red().bold(), "Not correct args!")
        };
    }
}