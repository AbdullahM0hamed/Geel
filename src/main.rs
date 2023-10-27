extern crate clap;
extern crate dirs;
extern crate meval;

#[macro_use]
extern crate crossterm;

mod inbuilt;
mod interpreter;
mod lexer;
mod parser;

use clap::Parser;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use dirs::home_dir;
use lexer::Lexer;
use std::env;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::{Stdout, Write, stdout};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    qoraal: Option<String>,

    #[arg(short, long)]
    kayd: Option<String>,

    #[arg(short, long)]
    caawimaad: bool,

    #[arg(short, long)]
    nooca: bool
}

fn show_help() {
    println!(r#"Isticmaalka: geel [DOORASHOOYIN]

Doorashooyinka:
  -q, --qoraal <QORAAL>
  -k, --kayd <KAYD>
  -n, --nooca                 Nooca ii sheeg
  -c, --caawimaad             I caawi"#)
}

macro_rules! get_version {
    () => { env!("CARGO_PKG_VERSION") };
}

fn get_home() -> String {
    let mut home = String::new();

    match home_dir() {
        Some(path) => {
            home = path.display().to_string();
        },
        None => {}
    }

    return home
}

fn write_history(text: &str) {
    let name = format!("{}/.geel_taariikh", get_home());
    let file = OpenOptions::new()
        .read(true)
        .open(&name);

    match file {
        Ok(_) => {
            let mut history = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&name)
                .unwrap();

            history.write_all(text.as_bytes()).unwrap();
        },
        Err(_) => {
            let history = File::create(&name);
            history.unwrap().write_all(text.as_bytes()).unwrap();
        }
    }
}

fn read_history(line_num: isize) -> (String, usize) {
    let name = format!("{}/.geel_taariikh", get_home());
    let binding = read_to_string(name).unwrap();
    let contents = binding.split("\r\n").filter(|&x|!x.is_empty());
    let lines = contents.collect::<Vec<&str>>();

    if line_num >= 0 && (line_num as usize) < lines.len() {
        return (lines[lines.len() - (line_num as usize) - 1].to_string(), lines.len());
    } else {
        return ("".to_string(), 0);
    }
}

fn has_history() -> bool {
    return Path::new(&format!("{}/.geel_taariikh", get_home())).exists();
}

fn insert(mut out: &Stdout, line: &mut String, offset: usize, c: char) {
    let remainder = &line.chars().collect::<String>()[line.len() - offset..line.len()];
    repl_print(
        out,
        &(c.to_string() + remainder)
    );

    if offset == 0 {
        repl_print(&out, &" ");
    }

    execute!(out, cursor::MoveLeft(offset as u16)).unwrap();
    line.insert(
        line.len() - offset,
        c
    );
}

fn move_right(mut out: &Stdout, offset: usize) {
    if offset != 0 {
        execute!(out, cursor::MoveRight(offset as u16)).unwrap();
    }
}

fn move_left(mut out: &Stdout, offset: usize) {
    if offset != 0 {
        execute!(out, cursor::MoveLeft(offset as u16)).unwrap();
    }
}

fn repl_print(mut out: &Stdout, text: &str) {
    execute!(out, Print(text)).unwrap();
}

fn repl() {
    println!("geel {}", get_version!());
    let mut code = String::new();
    let mut multiline: bool = false;
    let out = stdout();
    enable_raw_mode().unwrap();
    repl_print(&out, ">>> ");

    let mut line = String::new();
    let mut line_num: isize = -1;
    let mut offset = 0;
    loop {
        match read().unwrap() {
            Event::Key(event) => {
                if event.code == KeyCode::Backspace && line.len() != 0 && offset != line.len() {
                    move_right(&out, offset);
                    repl_print(&out, &"\x08 \x08".repeat(line.len()));
                    line.remove(line.len() - offset - 1);
                    repl_print(&out, &line);
                    move_left(&out, offset);
                } else if event.code == KeyCode::Enter {
                    offset = 0;
                    write_history(&format!("{}\r\n", line));
                    if line.chars().last() == Some(':') && !multiline {
                        multiline = true;
                        code.clear();
                    } else if line.trim().is_empty() {
                        multiline = false;
                    }

                    if multiline {
                        code = code + &line + "\n";
                        repl_print(&out, "\r\n... ");
                    } else {
                        repl_print(&out, "\r\n");
                        let parsed: Vec<parser::ParsedNode>;
                        if !code.is_empty() {
                            parsed = parser::Parser::new(lexer::Lexer::new(&code).lex()).parse();
                            code.clear();
                        } else {
                            parsed = parser::Parser::new(lexer::Lexer::new(&line).lex()).parse();
                        }

                        interpreter::Interpreter::new().interpret(true, parsed);
                        repl_print(&out, ">>> ");
                    }

                    line.clear();
                } else if event.code == KeyCode::Up {
                    if has_history() {
                        let new = read_history(line_num + 1);
                        if !new.0.is_empty() && line_num < new.1 as isize {
                            repl_print(
                                &out,
                             &"\x08 \x08".repeat(line.len())
                            );
                            line.clear();
                            line_num += 1;
                            line += &new.0;
                            repl_print(
                                &out,
                                &line
                            );
                        }
                    }
                } else if event.code == KeyCode::Down {
                    if has_history() {
                        repl_print(
                            &out,
                            &"\x08 \x08".repeat(line.len())
                        );
                        line.clear();

                        if line_num >= 0 {
                            line_num -= 1;
                            line += &read_history(line_num).0;
                            repl_print(
                                &out,
                                &line
                            );
                        }
                    }
                } else if event.code == KeyCode::Left {
                    if offset < line.len() {
                        execute!(&out, cursor::MoveLeft(1)).unwrap();
                        offset += 1;
                    }
                } else if event.code == KeyCode::Right {
                    if offset > 0 {
                        execute!(&out, cursor::MoveRight(1)).unwrap();
                        offset -= 1;
                    }
                } else if event.code == KeyCode::Home && offset < line.len() {
                    let change = line.len() - offset;
                    execute!(&out, cursor::MoveLeft(change as u16)).unwrap();
                    offset = line.len();
                } else if event.code == KeyCode::End /*&& offset > 0*/ {
                    move_right(&out, offset);
                    offset = 0;
                } else {
                    match event.modifiers {
                        KeyModifiers::NONE => {
                            match event.code {
                                KeyCode::Char(c) => {
                                    insert(&out, &mut line, offset, c);
                                },
                                _ => {}
                            }
                        },
                        KeyModifiers::SHIFT => {
                            match event.code {
                                KeyCode::Char(c) => {
                                    insert(&out, &mut line, offset, c);
                                },
                                _ => {}
                            }
                        }
                        KeyModifiers::CONTROL => {
                            match event.code {
                                KeyCode::Char('d') => {
                                    disable_raw_mode().unwrap();
                                    break
                                },
                                KeyCode::Char('c') =>  {
                                    repl_print(&out, &"\r\nWaaLaJoojiyey\r\n>>> ");
                                },
                                KeyCode::Char('u') => {
                                    move_right(&out, offset);
                                    repl_print(&out, &"\x08 \x08".repeat(line.len()));
                                    let _new = &line.chars().collect::<String>()[line.len() - offset..line.len()];
                                    repl_print(&out, &_new);
                                    move_left(&out, offset);
                                    line.clear();
                                    line += _new;
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1].chars().nth(0).unwrap() != '-' {
        println!("TODO: Compile Code");
    } else if args.len() == 1 {
        repl();
    } else {
        let c_args = Args::parse();

        if c_args.caawimaad {
            show_help();
        }

        if c_args.nooca {
            println!("Geel {}", get_version!());
        }

        if c_args.qoraal.is_some() {
            let qoraal = c_args.qoraal.unwrap();
            if !qoraal.is_empty() {
                println!("{:?}", Lexer::new(&qoraal).lex());
            } else {
                println!("Qoraal macno leh na sii.");
            }
        }

        if c_args.kayd.is_some() {
            let name = c_args.kayd.unwrap();
            if !name.is_empty() {
                if Path::new(&name).exists() {
                    let contents = read_to_string(name).unwrap();
                    println!("{:?}", parser::Parser::new(Lexer::new(&contents).lex()).parse());
                } else {
                    println!("Wax jirin baad noo tilmaamtey.");
                }
            } else {
                println!("Magac la'aan ma dhici karto.");
            }
        }
    }
}
