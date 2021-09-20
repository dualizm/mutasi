/**********************************************************
*  PROJECT_NAME: Mutasi;
*  VERSION: 0.2.5;
*  AUTHOR: Notidman;
***********************************************************/
mod game;
mod table;

use std::env;
use console::style;
use game::*;
use table::show_table;

fn main()
{

    let argv: Vec<String> = env::args().skip(1).collect();
    let mut iter = argv.into_iter().peekable();
    while let (Some(current), next) = (iter.next(), iter.peek()) {
        match current.as_str()
        {
            "--version" | "-v" => println!("{} {}", "Mutasi version", style("0.2.2").green().bold()),

            "--table" | "-t" => {

                if next == Some(&"b".to_string()) {
                    show_table("b")
                }
                else if next == Some(&"o".to_string()) {
                    show_table("o") 
                }
                else {
                    show_table("d")
                }
            },

            "--start" | "-s" => {
                if next == Some(&"b".to_string()) {
                    game("b")
                }
                else if next == Some(&"o".to_string()) {
                    game("o") 
                }
                else {
                    game("d")
                }
            },

            "--help" | "-h"  =>  println!("\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n",
            style("--help -h").red(),    " -> Show help commands.",
            style("--version -v").red(), " -> Show the version of the program.",
            style("--table -t").red(),   " -> Shows the multiplication table in decimal.",
            style("--table _ -t _").red(),   " -> Add a key for other number systems\n b(binary); o(octal).",
            style("--start -s").red(),   " -> Game with the multiplication table in decimal.",
            style("--start _ -s _").red(), " -> Add a key for other number systems\n b(binary); o(octal)."),

            // BETA Timer
            "--timer" => timer(),


            "o" | "b" => print!("{}", ""),
            _ => print!("Mutasi: {} {}", style("error:").red().bold(), "Not correct args!")
        };
    }
}
