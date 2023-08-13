use crate::backend::{Card, Number, Shape, Shading, Colour, Game, Move, new_game, draw_3, attempt_move, find_sets};

use std::io;

fn get_line() -> String {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            break input;
        }
        println!("Sorry, I don't understand. Please try again.")
    }
}

pub fn game_loop() {
    print!("Welcome to set! Would you like to play a game? (y/n): ");
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
    }
}

enum Choice {
    Draw3,
    Cheat,
    Move(Move),
}

fn get_move() -> Choice {
    'outer: loop {
        let ans = get_line();
        match ans.to_ascii_lowercase().get(0..1) {
            Some("p") => break Choice::Draw3,
            Some("l") => break Choice::Cheat,
            _ => (),
        }
        let mut mve = Move(1000, 1000, 1000);
        for i in 0..3 {
            let v = match ans.to_ascii_lowercase().get(i..(i+1)) {
                Some("q") => 0,
                Some("a") => 1,
                Some("z") => 2,
                Some("w") => 3,
                Some("s") => 4,
                Some("x") => 5,
                Some("e") => 6,
                Some("d") => 7,
                Some("c") => 8,
                Some("r") => 9,
                Some("f") => 10,
                Some("v") => 11,
                Some("t") => 12,
                Some("g") => 13,
                Some("b") => 14,
                Some("y") => 15,
                Some("h") => 16,
                Some("n") => 17,
                Some("u") => 18,
                Some("j") => 19,
                Some("m") => 20,
                _ => {
                    println!("Sorry, I don't understand. Please try again.");
                    continue 'outer;
                }
            };

            match i {
                0 => mve.0 = v,
                1 => mve.1 = v,
                2 => mve.2 = v,
                _ => unreachable!(),
            }
        }
        break Choice::Move(mve);
    }
}

fn play_game() {
    let mut game = new_game();

    // Main game loop
    'outer: loop {
        print_game(&game);
        let mve = match get_move() {
            Choice::Draw3 => {
                match draw_3(game) {
                    Ok(g) => {
                        game = g;
                        continue 'outer;
                    }
                    Err(g) => {
                        println!("There aren't enough cards in the deck. Game over!");
                        break 'outer;
                    }
                }
            }
            Choice::Cheat => {
                let sets = find_sets(&game);
                if sets.len() == 0 {
                    println!("There are no sets on the board. You have to draw 3 cards.");
                } else {
                    if sets.len() == 1 {
                        println!("There is 1 set on the board:");
                    } else {
                        println!("There are {} sets on the board:", sets.len());
                    }
                    for s in sets.iter() {
                        println!("{} {} {}", card_string(&game.in_play[s.0]), card_string(&game.in_play[s.1]), card_string(&game.in_play[s.2]));
                    }
                }
                continue 'outer;
            }
            Choice::Move(m) => m,
        };

        match attempt_move(game, &mve) {
            Ok(g) => {
                game = g;
            }
            Err((g, p)) => {
                println!("Not a valid set! Try again!");
                game = g;
            }
        }
    }
}

fn print_game(g: &Game) {
    println!("Score: {}", g.score);
    println!("Cards left in deck: {}", g.deck.len());
    println!();

    // Print the cards in three equal lines
    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();

    for (i, c) in g.in_play.iter().enumerate() {
        let card_str = card_string(c);
        match i % 3 {
            0 => {
                line1.push_str(&card_str);
                line1.push_str(" ");
            }
            1 => {
                line2.push_str(&card_str);
                line2.push_str(" ");
            }
            2 => {
                line3.push_str(&card_str);
                line3.push_str(" ");
            }
            _ => unreachable!(),
        }
    }

    println!("{}", line1);
    println!("{}", line2);
    println!("{}", line3);

    println!();
}

fn card_string(c: &Card) -> String {
    let mut out = String::new();
    match c.number {
        Number::One => out.push_str("1"),
        Number::Two => out.push_str("2"),
        Number::Three => out.push_str("3"),
    }
    match c.shape {
        Shape::Oval => out.push_str("O"),
        Shape::Wave => out.push_str("W"),
        Shape::Diamond => out.push_str("D"),
    }
    match c.shading {
        Shading::Empty => out.push_str("E"),
        Shading::Half => out.push_str("H"),
        Shading::Full => out.push_str("F"),
    }
    match c.colour {
        Colour::Red => out.push_str("R"),
        Colour::Green => out.push_str("G"),
        Colour::Purple => out.push_str("P"),
    }

    out
}
