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
use table::Table;

fn main()
{
    let mut iter = env::args().skip(1).into_iter().peekable();
    while let (Some(current), next) = (iter.next(), iter.peek()) {
        match current.as_str()
        {
            "--version" | "-v" => println!("{} {}", "Mutasi version", style("0.2.5").green().bold()),

            "--table" | "-t" => {

                if next == Some(&"b".to_string()) {
                    Table::show_table("b")
                }
                else if next == Some(&"o".to_string()) {
                    Table::show_table("o") 
                }
                else {
                    Table::show_table("d")
                }
            },

            "--start" | "-s" => {
                if next == Some(&"b".to_string()) {
                    let mut gm = Game::new("b");
                    gm.game();
                }
                else if next == Some(&"o".to_string()) {
                    let mut gm = Game::new("o");
                    gm.game(); 
                }
                else {
                    let mut gm = Game::new("d");
                    gm.game();
                }
            },

            "--help" | "-h"  =>  println!("\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n",
            style("--help -h").red(),    " -> show help commands.",
            style("--version -v").red(), " -> show the version of the program.",
            style("--table -t").red(),   " -> shows the multiplication table in decimal.",
            style("--table _ -t _").red(),   " -> add a key for other number systems\n b(binary); o(octal).",
            style("--start -s").red(),   " -> game with the multiplication table in decimal.",
            style("--start _ -s _").red(), " -> add a key for other number systems\n b(binary); o(octal).",
            style("--timer").red(),   " -> shows a timer of the form mm:ss.",
            style("--byterush").red(), " -> launches a complex binary multiplication game."),

            // BETA Timer
            "--timer" => timer(),

            "--byterush" => {
                let mut gm = Game::new("r");
                gm.game();
            },


            "o" | "b" => print!("{}", ""),
            _ => print!("Mutasi: {} {}", style("error:").red().bold(), "Not correct args!")
        };
    }
}
