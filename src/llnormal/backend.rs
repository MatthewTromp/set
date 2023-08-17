use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Number {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Oval,
    Wave,
    Diamond,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shading {
    Empty,
    Half,
    Full,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    Red,
    Green,
    Purple,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    number: Number,
    shape: Shape,
    shading: Shading,
    colour: Colour,
}

impl Card {
    pub fn new(number: Number, shape: Shape, shading: Shading, colour: Colour) -> Self {
        Card {number, shape, shading, colour}
    }

    pub fn get_number(&self) -> Number {
        self.number
    }

    pub fn get_shape(&self) -> Shape {
        self.shape
    }

    pub fn get_shading(&self) -> Shading {
        self.shading
    }

    pub fn get_colour(&self) -> Colour {
        self.colour
    }
}


#[derive(Debug)]
pub struct Game {
    deck: Vec<Card>,
    in_play: Vec<Card>,
    score: i32,
}

// Move that is within the bounds of the game: all indices refer to cards that are actually in play
pub struct ValidMove<'a> {
    game: &'a Game,
    c1: usize,
    c2: usize,
    c3: usize,
}

// A move that actually produces a set in the game
pub struct SetMakingMove<'a> {
    game: &'a Game,
    mve: ValidMove<'a>,
}

// Card that is currently in play
pub struct PlayedCard<'a> {
    game: &'a Game,
    card: Card,
}

impl PlayedCard<'_> {
    fn new<'a, 'b>(game: &'a Game, card: &'b Card) -> PlayedCard<'a> {
        PlayedCard {game, card: card.clone()}
    }

    pub fn get_card(&self) -> &Card {
        &self.card
    }
}

pub struct InvalidMoveError;

impl Game {
    pub fn remaining_cards(&self) -> usize {
        self.deck.len()
    }

    pub fn cards_in_play(&self) -> Vec<PlayedCard> {
        (&self).in_play.iter().map(|c| {PlayedCard::new(&self, c)}).collect()
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn check_move_bounds<'a>(&'a self, c1: usize, c2: usize, c3: usize) -> Result<ValidMove<'a>, InvalidMoveError> {
    }
        
}


pub fn new_game() -> Game {
    let mut game = Game {
        deck: make_shuffled_deck(),
        in_play: vec![],
        score: 0
    };

    // Draw until we have 12 cards
    for _ in 0..12 {
        game.in_play.push(game.deck.pop().expect("Always have 12 cards in fresh deck"));
    }

    game
}

// draw 3 new cards for when the user can't find any more cards
pub fn draw_3(mut g: Game) -> Result<Game, Game> {
    for _ in 0..3 {
        match g.deck.pop() {
            Some(c) => g.in_play.push(c),
            None => return Err(g),
        }
    }

    // Deduct a point from the score
    g.score -= 1;

    Ok(g)
}

pub enum PlayError {
    InvalidMove,
    NotASet,
}

pub fn attempt_move(mut g: Game, m: Move) -> Result<Game, (Game, PlayError)> {
    if (&g as *const Game) != (m.game as *const Game) {
        panic!("Cannot use move from different game");
    }
    
    let (c1, c2, c3) = (m.c1, m.c2, m.c3);
    if (c1 == c2) || (c2 == c3) || (c1 == c3) || (c1 >= g.in_play.len()) || (c2 >= g.in_play.len()) || (c3 >= g.in_play.len()) {
        return Err((g, PlayError::InvalidMove));
    }

    let card1 = g.in_play[c1];
    let card2 = g.in_play[c2];
    let card3 = g.in_play[c3];

    if !is_set(&card1, &card2, &card3) {
        g.score -= 1;
        return Err((g, PlayError::NotASet));
    }
    
    g.score += 1;
    if g.deck.len() == 0 {
        // No more cards in deck. Remove cards from play and that's it
        let mut v = vec![c1, c2, c3];
        v.sort();
        v.iter().rev().for_each(|&i| {
            g.in_play.remove(i as usize);
        });
    } else if g.in_play.len() <= 12 {
        assert_eq!(g.in_play.len(), 12);
        // Normal play. Replace cards in-place
        vec![c1, c2, c3].iter().for_each(|&i| {
            g.in_play[i] = g.deck.pop().expect("Impossible to fail to draw a card: deck always has multiple of 3 cards");
        });
    } else {
        // Too many cards. Do not replace

        // Remove cards and replace them with cards from the end (minimize card movements)
        // Here's how this is going to work:
        // 1. figure out where the end of the play area that's still going to exist is (current len - 3)
        // 2. remove all cards that are part of a set past that point
        // 3. Swap out the cards before the new end with those past the new end

        let new_max = g.in_play.len() - 3;

        let mut pre = vec![];
        let mut post = vec![];

        for i in [c1, c2, c3] {
            if i < new_max {
                pre.push(i);
            } else {
                post.push(i);
            }
        }

        // Sort post from largest to smallest (so that removals don't change positions of other cards to be removed)
        post.sort();
        post.reverse();

        for i in post {
            g.in_play.remove(i);
        }

        // Replace cards before the break with cards from the end
        for i in pre {
            g.in_play[i] = g.in_play.pop().expect("Deck cannot be empty");
        }

        assert_eq!(g.in_play.len() % 3, 0);
    }

    Ok(g)
}

pub fn find_sets(g: &Game) -> Vec<Move> {
    // TODO: could maybe be more efficient with a set? But probably not just because of the overhead of a hashset lookup
    // Could probably be made more efficient with some bithacking nonsense (u128)
    let mut sets = vec![];

    for i in 0..g.in_play.len() {
        for j in (i+1)..g.in_play.len() {
            for k in (j+1)..g.in_play.len() {
                if is_set(&g.in_play[i], &g.in_play[j], &g.in_play[k]) {
                    sets.push(Move{game: &g, c1: i, c2: j, c3: k});
                }
            }
        }
    }

    sets
}

fn make_deck() -> Vec<Card> {
    // Make the deck
    let mut deck = Vec::with_capacity(81);
    for number in vec![Number::One, Number::Two, Number::Three] {
        for shape in vec![Shape::Oval, Shape::Wave, Shape::Diamond] {
            for shading in vec![Shading::Empty, Shading::Half, Shading::Full] {
                for colour in vec![Colour::Red, Colour::Green, Colour::Purple] {
                    deck.push(Card::new(number, shape, shading, colour));
                }
            }
        }
    }

    deck
}

fn make_shuffled_deck() -> Vec<Card> {
    let mut deck = make_deck();

    // Shuffle the deck
    deck.shuffle(&mut thread_rng());

    deck
}

fn is_set(card1: &Card, card2: &Card, card3: &Card) -> bool {
    (
	    (card1.number == card2.number && card2.number == card3.number) || (card1.number != card2.number && card2.number != card3.number && card3.number != card1.number)
    ) && (
	    (card1.shape == card2.shape && card2.shape == card3.shape) || (card1.shape != card2.shape && card2.shape != card3.shape && card3.shape != card1.shape)
    ) && (
	    (card1.shading == card2.shading && card2.shading == card3.shading) || (card1.shading != card2.shading && card2.shading != card3.shading && card3.shading != card1.shading)
    ) && (
	    (card1.colour == card2.colour && card2.colour == card3.colour) || (card1.colour != card2.colour && card2.colour != card3.colour && card3.colour != card1.colour)
    )
}

pub fn has_set(cards: &[Card]) -> bool {
    panic!("Not implemented!");
}

#[cfg(test)]
mod tests {
 
    use super::{Number::*, Shape::*, Shading::*, Colour::*};
    use super::*;
    
    #[test]
    fn is_set_diff_num() {
        assert!(is_set(&Card::new(One, Oval, Empty, Red), &Card::new(Two, Oval, Empty, Red), &Card::new(Three, Oval, Empty, Red)));
        assert!(is_set(&Card::new(Three, Oval, Empty, Red), &Card::new(One, Oval, Empty, Red), &Card::new(Two, Oval, Empty, Red)));
    }

    #[test]
    fn is_set_diff_shape() {
	    assert!(is_set(&Card::new(One, Oval, Empty, Red), &Card::new(One, Wave, Empty, Red), &Card::new(One, Diamond, Empty, Red)));
    }

    #[test]
    fn is_set_diff_shading() {
	    assert!(is_set(&Card::new(Two, Diamond, Half, Green), &Card::new(Two, Diamond, Empty, Green), &Card::new(Two, Diamond, Full, Green)));
    }

    #[test]
    fn is_set_diff_colour() {
	    assert!(is_set(&Card::new(Two, Diamond, Half, Green), &Card::new(Two, Diamond, Half, Red), &Card::new(Two, Diamond, Half, Purple)));
    }

    #[test]
    fn is_set_all_diff() {
	    assert!(is_set(&Card::new(One, Oval, Half, Green), &Card::new(Two, Wave, Empty, Purple), &Card::new(Three, Diamond, Full, Red)));
    }

    #[test]
    fn is_set_wrong_num() {
	    assert!(!is_set(&Card::new(One, Oval, Empty, Red), &Card::new(One, Wave, Empty, Red), &Card::new(Two, Diamond, Empty, Red)));
    }

    #[test]
    fn is_set_wrong_shape() {
	    assert!(!is_set(&Card::new(One, Oval, Half, Green), &Card::new(Two, Wave, Empty, Purple), &Card::new(Three, Wave, Full, Red)));
    }

    #[test]
    fn test_make_deck() {
        let deck = make_deck();
        assert_eq!(deck.len(), 81);
        assert_eq!(deck.iter().filter(|&&c| c == Card::new(One, Oval, Half, Purple)).count(), 1);
        assert_eq!(deck.iter().filter(|&&c| c == Card::new(Three, Wave, Full, Green)).count(), 1);
        assert_eq!(deck.iter().filter(|&&c| c == Card::new(Two, Diamond, Empty, Red)).count(), 1);
    }

    #[test]
    fn test_make_shuffled_deck() {
        let deck = make_shuffled_deck();
        assert_eq!(deck.len(), 81);
        assert_eq!(deck.iter().filter(|&&c| c == Card::new(One, Oval, Half, Purple)).count(), 1);
        assert_eq!(deck.iter().filter(|&&c| c == Card::new(Three, Wave, Full, Green)).count(), 1);
        assert_eq!(deck.iter().filter(|&&c| c == Card::new(Two, Diamond, Empty, Red)).count(), 1);
    }
}
