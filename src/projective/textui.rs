use super::backend::{Move, CardPos, Game, PlayError, attempt_move, Card, MoveBuildError};

use std::io;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn get_line() -> String {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            break input.trim().to_string();
        }
        println!("Sorry, I don't understand. Please try again.")
    }
}

pub fn game_loop() {
    print!("Welcome to projective set! Would you like to play a game? (y/n): ");
    io::stdout().flush().unwrap();
    'outer: loop {
        loop {
            let ans = get_line();
            match ans.to_ascii_lowercase().get(0..1) {
                Some("y") => {
                    println!("Great!");
                    break;
                }
                Some("n") => {
                    println!("Alright! Another time maybe :>");
                    break 'outer;
                } 
                None | Some(_) => {
                    println!("Sorry, I don't understand. Please try again.")
                }
            }
        }

        play_game();

        print!("That was fun! Would you like to play again? (y/n): ");
        io::stdout().flush().unwrap();
    }
}

enum Choice {
    Move(Move),
    Quit,
}
fn get_move() -> Choice {
    loop {
        let ans = get_line();
        if let Some('q') = ans.chars().next() {
            break Choice::Quit
        }
        match ans.chars().map(|c| {
            match c {
                '1' => Ok(CardPos::build(0).unwrap()),
                '2' => Ok(CardPos::build(1).unwrap()),
                '3' => Ok(CardPos::build(2).unwrap()),
                '4' => Ok(CardPos::build(3).unwrap()),
                '5' => Ok(CardPos::build(4).unwrap()),
                '6' => Ok(CardPos::build(5).unwrap()),
                '7' => Ok(CardPos::build(6).unwrap()),
                _ => Err(()),
            }
        }).collect() {
            Ok(v) => {
                match Move::build(v) {
                    Ok(m) => break Choice::Move(m),
                    Err(MoveBuildError::RepeatCards) => println!("Make sure you only choose each card once!"),
                    Err(MoveBuildError::InsufficientCards) => println!("You have to choose at least 3 cards!"),
                }
            }
            Err(()) => println!("Sorry, I don't understand"),
        }
    }
}

fn play_game() {
    let mut game = Game::new();

    // Main game loop
    loop {
        print_game(&game);
        match get_move() {
            Choice::Move(mve) => {
                match attempt_move(game, &mve) {
                    Ok(g) => {
                        game = g;
                    }
                    Err((g, p)) => {
                        match p {
                            PlayError::NotASet => println!("Not a valid set! Try again!"),
                            PlayError::InvalidMove => println!("Invalid move! Try again!"),
                        }
                        game = g;
                    }
                }
            }
            Choice::Quit => break,
        }
    }   
}

fn print_game(game: &Game) {
    for (e, card) in game.in_play().iter().enumerate() {
        print!("{}.", e+1);
        print_card(card);
        println!();
    }
}


fn print_card(card: &Card) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if card.is_red() {
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Red)));
    } else {
        stdout.set_color(ColorSpec::new().set_bg(None));
    }
    write!(&mut stdout, " ");
    if card.is_orange() {
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Cyan)));
    } else {
        stdout.set_color(ColorSpec::new().set_bg(None));
    }
    write!(&mut stdout, " ");
    if card.is_yellow() {
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Yellow)));
    } else {
        stdout.set_color(ColorSpec::new().set_bg(None));
    }
    write!(&mut stdout, " ");
    if card.is_green() {
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Green)));
    } else {
        stdout.set_color(ColorSpec::new().set_bg(None));
    }
    write!(&mut stdout, " ");
    if card.is_blue() {
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Blue)));
    } else {
        stdout.set_color(ColorSpec::new().set_bg(None));
    }
    write!(&mut stdout, " ");
    if card.is_purple() {
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Magenta)));
    } else {
        stdout.set_color(ColorSpec::new().set_bg(None));
    }
    write!(&mut stdout, " ");
    stdout.set_color(ColorSpec::new().set_bg(None));
}
