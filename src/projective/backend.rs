use std::collections::HashSet;
use std::fmt;
use std::error::Error;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Card {
    red: bool,
    orange: bool,
    yellow: bool,
    green: bool,
    blue: bool,
    purple: bool,
}

impl Card {
    fn from_int(i: usize) -> Self {
        if i == 0 || i >= 64 {
            panic!("Invalid card number!");
        }
        Card {
            red: ((i & 32) > 0),
            orange: ((i & 16) > 0),
            yellow: ((i & 8) > 0),
            green: ((i & 4) > 0),
            blue: ((i & 2) > 0),
            purple: ((i & 1) > 0),
        }
    }

    pub fn is_red(&self) -> bool {
        self.red
    }

    pub fn is_orange(&self) -> bool {
        self.orange
    }

    pub fn is_yellow(&self) -> bool {
        self.yellow
    }

    pub fn is_green(&self) -> bool {
        self.green
    }

    pub fn is_blue(&self) -> bool {
        self.blue
    }

    pub fn is_purple(&self) -> bool {
        self.purple
    }
}

pub struct Game {
    deck: Vec<Card>,
    in_play: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        let mut out = Game {
            deck: make_shuffled_deck(),
            in_play: vec![],
        };
        for _ in 0..7 {
            out.in_play.push(out.deck.pop().unwrap());
        }
        out
    }

    pub fn in_play(&self) -> &[Card] {
        &self.in_play[..]
    }
}

#[derive(Debug)]
pub struct CardPosError;

impl fmt::Display for CardPosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card position must be between 0 and 6 (inclusive)")
    }
}

impl Error for CardPosError {}

#[derive(Copy, Clone)]
pub struct CardPos(u8);

impl CardPos {
    pub fn build(i: u8) -> Result<Self, CardPosError> {
        if i > 6 {
            return Err(CardPosError);
        }

        Ok(CardPos(i))
    }

    pub fn get_index(&self) -> u8 {
        self.0
    }
}

#[derive(Debug)]
pub enum MoveBuildError {
    RepeatCards,
    InsufficientCards,
}

impl fmt::Display for MoveBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveBuildError::RepeatCards => write!(f, "Repeated card(s)"),
            MoveBuildError::InsufficientCards => write!(f, "Insufficient cards (need at least 3)"),
        }
    }
}

impl Error for MoveBuildError {}
        
pub struct Move(Vec<CardPos>);

impl Move {
    pub fn build(mut v: Vec<CardPos>) -> Result<Self, MoveBuildError> {
        // Make sure there are enough cards
        if v.len() < 3 {
            return Err(MoveBuildError::InsufficientCards);
        }

        // Make sure all cards are unique
        let h: HashSet<u8> = v.iter().map(|CardPos(c)| *c).collect();
        if h.len() != v.len() {
            return Err(MoveBuildError::RepeatCards);
        }

        // Sort in reverse order to ensure cards are removed in correct order
        v.sort_by_key(|CardPos(i)| {-(*i as i8)});
        Ok(Move(v))
    }
}

fn make_deck() -> Vec<Card> {
    (1..64).map(Card::from_int).collect()
}

fn make_shuffled_deck() -> Vec<Card> {
    let mut deck = make_deck();

    // Shuffle the deck
    deck.shuffle(&mut thread_rng());

    deck
}

pub enum PlayError {
    InvalidMove,
    NotASet,
}

pub fn attempt_move(mut g: Game, mve: &Move) -> Result<Game, (Game, PlayError)> {
    match is_set(&g, mve) {
        Ok(true) => (),
        Ok(false) => return Err((g, PlayError::NotASet)),
        Err(p) => return Err((g, p)),
    };

    for CardPos(i) in &mve.0 {
        if let Some(c) = g.deck.pop() {
            g.in_play[*i as usize] = c;
        } else {
            g.in_play.remove(*i as usize);
        }
    }

    return Ok(g);
       
}

fn is_set(g: &Game, mve: &Move) -> Result<bool, PlayError> {
    let mut red_count = 0;
    let mut orange_count = 0;
    let mut yellow_count = 0;
    let mut green_count = 0;
    let mut blue_count = 0;
    let mut purple_count = 0;

    for CardPos(i) in mve.0.iter() {
        if (*i as usize) >= g.in_play.len() {
            return Err(PlayError::InvalidMove);
        }
        let c = &g.in_play[*i as usize];

        if c.is_red() {red_count += 1;}
        if c.is_orange() {orange_count += 1;}
        if c.is_yellow() {yellow_count += 1;}
        if c.is_green() {green_count += 1;}
        if c.is_blue() {blue_count += 1;}
        if c.is_purple() {purple_count += 1;}
    }

    let valid = [red_count, orange_count, yellow_count, green_count, blue_count, purple_count].iter().map(|v| {v % 2 == 0}).fold(true, |acc, i| {acc && i});

    Ok(valid)
} 
