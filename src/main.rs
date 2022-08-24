use std::io::{stdin, stdout, Write};

use sudoku_solver::Sudoku;

fn main() {
    println!("Type 'help' for a list of all commands.\n");
    let mut sudoku = Sudoku::new();

    loop {
        let input = get_input();
        let words: Vec<&str> = input.split(' ').collect();

        match words[0] {
            "new" => {
                sudoku = Sudoku::new();
            }
            "add" => {
                if words.len() < 4 {
                    explain_add();
                    continue;
                }

                match words[1].parse::<usize>() {
                    Ok(x) => match words[2].parse::<usize>() {
                        Ok(y) => match words[3].parse::<u8>() {
                            Ok(number) => {
                                sudoku.set_value(x - 1, y - 1, number);
                                println!();
                            }
                            Err(_) => {
                                explain_add();
                                continue;
                            }
                        },
                        Err(_) => {
                            explain_add();
                            continue;
                        }
                    },
                    Err(_) => {
                        explain_add();
                        continue;
                    }
                }
            }
            "solve" => {
                sudoku.solve();
            }
            "print" => {
                sudoku.print();
            }
            "exit" => {
                break;
            }
            "help" => {
                help();
            }
            _ => {
                println!("Invalid command.\n");
                continue;
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

fn explain_add() {
    println!(
        "Correct usage of 'add':\n
        add <row> <column> <number to insert>\n
Numbers must be between 1 and 9.\n"
    );
}

fn help() {
    println!("\n\tadd <row> <column> <number to insert>\n\t\tNumbers must be between 1 and 9.\n");
    println!("\texit");
    println!("\tnew");
    println!("\tprint");
    println!("\tsolve");
}
