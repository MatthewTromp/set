use super::backend::{Card, Number, Shape, Shading, Colour as CardColour, Game, Move, new_game, draw_3, attempt_move, find_sets, PlayError};

use std::io;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    Draw3,
    Cheat,
    Move(usize, usize, usize),
}

fn letter_to_index(letter: char) -> Result<usize, ()> {
    match letter.to_lowercase().next().unwrap() {
        'q' => Ok(0),
        'a' => Ok(1),
        'z' => Ok(2),
        'w' => Ok(3),
        's' => Ok(4),
        'x' => Ok(5),
        'e' => Ok(6),
        'd' => Ok(7),
        'c' => Ok(8),
        'r' => Ok(9),
        'f' => Ok(10),
        'v' => Ok(11),
        't' => Ok(12),
        'g' => Ok(13),
        'b' => Ok(14),
        'y' => Ok(15),
        'h' => Ok(16),
        'n' => Ok(17),
        'u' => Ok(18),
        'j' => Ok(19),
        'm' => Ok(20),
        _ => Err(()),
    }
}

fn get_move() -> Choice {
    loop {
        let ans = get_line();
        match ans.to_ascii_lowercase().get(0..1) {
            Some("p") => break Choice::Draw3,
            Some("l") => break Choice::Cheat,
            _ => (),
        }
        match ans.chars().take(3).map(letter_to_index).collect::<Result<Vec<usize>, ()>>() {
            Err(()) => println!("Sorry, I don't understand. Please try again."),
            Ok(v) => {
                if v.len() != 3 {
                    println!("Must specify 3 cards");
                } else {
                    break Choice::Move(v[0], v[1], v[2]);
                }
            }
        }
    }
}

fn play_game() {
    let mut game = new_game();

    // Main game loop
    'outer: loop {
        print_game(&game);
        let mve = match get_move() {
            Choice::Draw3 => {
                if game.cards_in_play().len() == 21 {
                    println!("Can't have more than 21 cards in play! (hint: there's guaranteed to be a set here)");
                    continue;
                }
                match draw_3(game) {
                    Ok(g) => {
                        game = g;
                        continue 'outer;
                    }
                    Err(g) => {
                        game = g;
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
                        println!("{} {} {}", card_string(&game.cards_in_play()[s.0]), card_string(&game.cards_in_play()[s.1]), card_string(&game.cards_in_play()[s.2]));
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
                match p {
                    PlayError::NotASet => println!("Not a valid set! Try again!"),
                    PlayError::InvalidMove => println!("Invalid move! Try again!"),
                }
                game = g;
            }
        }
    }
}

const DIAMOND_EMPTY: [&str; 11] =
[r"          ",
 r"    /\    ",
 r"   /  \   ",
 r"  /    \  ",
 r" /      \ ",
 r"/        \",
 r"\        /",
 r" \      / ",
 r"  \    /  ",
 r"   \  /   ",
 r"    \/    "];

const DIAMOND_HALF: [&str; 11] =
[r"          ",
 r"    /\    ",
 r"   /--\   ",
 r"  /    \  ",
 r" /------\ ",
 r"/        \",
 r"\--------/",
 r" \      / ",
 r"  \----/  ",
 r"   \  /   ",
 r"    \/    "];

const DIAMOND_FULL: [&str; 11] =
[r"          ",
 r"    /\    ",
 r"   /--\   ",
 r"  /----\  ",
 r" /------\ ",
 r"/--------\",
 r"\--------/",
 r" \------/ ",
 r"  \----/  ",
 r"   \--/   ",
 r"    \/    "];

const OVAL_EMPTY: [&str; 11] =
[r"  ______  ",
 r" /      \ ",
 r"|        |",
 r"|        |",
 r"|        |",
 r"|        |",
 r"|        |",
 r"|        |",
 r"|        |",
 r"|        |",
 r" \______/ "];


const OVAL_HALF: [&str; 11] =
[r"  ______  ",
 r" /      \ ",
 r"|--------|",
 r"|        |",
 r"|--------|",
 r"|        |",
 r"|--------|",
 r"|        |",
 r"|--------|",
 r"|        |",
 r" \000000/ "];

const OVAL_FULL: [&str; 11] =
[r"  ______  ",
 r" /------\ ",
 r"|--------|",
 r"|--------|",
 r"|--------|",
 r"|--------|",
 r"|--------|",
 r"|--------|",
 r"|--------|",
 r"|--------|",
 r" \000000/ "];

const WAVE_EMPTY: [&str; 11] =
[r"  _____   ",
 r" /     \  ",
 r" \      \ ",
 r"  \      \",
 r"   |     /",
 r"  /     / ",
 r" /     /  ",
 r"/     |   ",
 r"\      \  ",
 r" \      \ ",
 r"  \_____/ "];

const WAVE_HALF: [&str; 11] =
[r"  _____   ",
 r" /     \  ",
 r" \------\ ",
 r"  \      \",
 r"   |-----/",
 r"  /     / ",
 r" /-----/  ",
 r"/     |   ",
 r"\------\  ",
 r" \      \ ",
 r"  \00000/ "];

const WAVE_FULL: [&str; 11] =
[r"  _____   ",
 r" /-----\  ",
 r" \------\ ",
 r"  \------\",
 r"   |-----/",
 r"  /-----/ ",
 r" /-----/  ",
 r"/-----|   ",
 r"\------\  ",
 r" \------\ ",
 r"  \00000/ "];


fn print_game(g: &Game) {
    println!("Score: {}", g.get_score());
    println!("Cards left in deck: {}", g.remaining_cards());
    println!();

    // Print the cards in three equal lines
    let mut line1: Vec<Card> = Vec::new();
    let mut line2: Vec<Card> = Vec::new();
    let mut line3: Vec<Card> = Vec::new();

    for (i, c) in g.cards_in_play().iter().enumerate() {
        match i % 3 {
            0 => {
                line1.push(*c);
            }
            1 => {
                line2.push(*c);
            }
            2 => {
                line3.push(*c);
            }
            _ => unreachable!(),
        }
    }

    print_cards_pretty(&line1);
    println!();
    print_cards_pretty(&line2);
    println!();
    print_cards_pretty(&line3);
    println!();
    
    println!();
}

// Shapes are 11 lines x 10 characters

// Cards are 46 characters long. 44 characters of internal space. 4 characters between adjacent shapes (for 3 shapes: 2*3 spaces of padding from edge, 2*4 spaces of padding between shapes, 3*10 characters of shape. 6+8+30=44)
// Cards have 5 characters of space between them
const INTER_CARD_SPACE: usize = 5;
const INTER_SHAPE_SPACE: usize = 4;
const SHAPE_WIDTH: usize = 10;
const CARD_WIDTH: usize = 46;
const INSIDE_CARD_WIDTH: usize = CARD_WIDTH-2;
const SHAPE_LINES: usize = 11;
    
fn print_cards_pretty(cards: &[Card]) {

    let num_cards = cards.len();
    
    let card_top_and_bot: String = "+".to_string() + &"-".repeat(INSIDE_CARD_WIDTH) + "+";
    let print_card_top_or_bot = move || {
        print!("{}", card_top_and_bot);
        for _ in 0..(num_cards-1) {
            print!("{}", &" ".repeat(INTER_CARD_SPACE));
            print!("{}", card_top_and_bot);
        }
    };
    
    // Write tops of cards
    print_card_top_or_bot();
    println!();
    

    for l in 0..SHAPE_LINES {
        for card in cards {
            print_card_line_pretty(card, l);
            print!("{}", &" ".repeat(INTER_CARD_SPACE));
        }
        println!();
    }

    // Extra line of space
    let empty_line = "|".to_string() + &" ".repeat(INSIDE_CARD_WIDTH) + "|";
    print!("{}", empty_line);
    for _ in 0..(num_cards-1) {
        print!("{}", &" ".repeat(INTER_CARD_SPACE));
        print!("{}", empty_line);
    }
    println!();

    // Write bottoms of cards
    print_card_top_or_bot();
    println!();
}

fn print_card_line_pretty(card: &Card, line: usize) {
    use {Shape::*, Shading::*, Number::*, CardColour::*};
    let template = match (card.get_shape(), card.get_shading()) {
        (Diamond, Empty) => DIAMOND_EMPTY,
        (Diamond, Half) => DIAMOND_HALF,
        (Diamond, Full) => DIAMOND_FULL,
        (Oval, Empty) => OVAL_EMPTY,
        (Oval, Half) => OVAL_HALF,
        (Oval, Full) => OVAL_FULL,
        (Wave, Empty) => WAVE_EMPTY,
        (Wave, Half) => WAVE_HALF,
        (Wave, Full) => WAVE_FULL,
    };

    let shape_count = match card.get_number() {
        One => 1,
        Two => 2,
        Three => 3,
    };

    let color = match card.get_colour() {
        Red => Color::Red,
        Green => Color::Green,
        Purple => Color::Magenta,
    };

    let outside_padding_size = (INSIDE_CARD_WIDTH - shape_count * SHAPE_WIDTH - (shape_count - 1) * INTER_SHAPE_SPACE)/2;

    let outside_padding = " ".repeat(outside_padding_size);
    print!("|{}", outside_padding);


    // Extract relevant line of template and print
    let strng = template[line];

    print_shape_line(strng, &color);
    for _ in 0..(shape_count-1) {
        print!("{}", &" ".repeat(INTER_SHAPE_SPACE));
        print_shape_line(strng, &color);
    }

    print!("{}|", outside_padding)
}

fn print_shape_line(s: &str, color: &Color) {
    
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut bg_on = false;
    stdout.set_color(ColorSpec::new().set_fg(Some(*color))).unwrap();

    for c in s.chars() {
        let (chr, has_bg) = match c {
            '0' => ('_', true),
            '-' => (' ', true),
            c => (c, false),
        };

        match (has_bg, bg_on) {
            (true, false) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(*color)).set_bg(Some(*color))).unwrap();
                bg_on = true;
            }
            (false, true) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(*color)).set_bg(None)).unwrap();
                bg_on = false;
            },
            (_, _) => (),
        }

        write!(&mut stdout, "{chr}").unwrap();
    }
    stdout.set_color(ColorSpec::new().set_fg(None).set_bg(None)).unwrap();
}
                      
fn card_string(c: &Card) -> String {
    let mut out = String::new();
    match c.get_number() {
        Number::One => out.push_str("1"),
        Number::Two => out.push_str("2"),
        Number::Three => out.push_str("3"),
    }
    match c.get_shape() {
        Shape::Oval => out.push_str("O"),
        Shape::Wave => out.push_str("W"),
        Shape::Diamond => out.push_str("D"),
    }
    match c.get_shading() {
        Shading::Empty => out.push_str("E"),
        Shading::Half => out.push_str("H"),
        Shading::Full => out.push_str("F"),
    }
    match c.get_colour() {
        CardColour::Red => out.push_str("R"),
        CardColour::Green => out.push_str("G"),
        CardColour::Purple => out.push_str("P"),
    }

    out
}


