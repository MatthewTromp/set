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
    pub number: Number,
    pub shape: Shape,
    pub shading: Shading,
    pub colour: Colour,
}

impl Card {
    pub fn new(number: Number, shape: Shape, shading: Shading, colour: Colour) -> Self {
        Card {number, shape, shading, colour}
    }
}

#[derive(Debug)]
pub struct Game {
    pub deck: Vec<Card>,
    pub in_play: Vec<Card>,
    pub score: i32,
}

pub struct Move(pub usize, pub usize, pub usize);

pub fn new_game() -> Game {
    let game = Game {
        deck: make_shuffled_deck(),
        in_play: vec![],
        score: 0
    };

    // Draw until we have 12 cards
    let game = draw_until_12(game).expect("Impossible to fail to draw 12 cards from freshly made deck");

    game
}

fn draw_until_12(mut g: Game) -> Result<Game, Game> {
    while g.in_play.len() < 12 {
        match g.deck.pop() {
            Some(c) => g.in_play.push(c),
            None => return Err(g),
        }
    }

    Ok(g)
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

pub fn attempt_move(mut g: Game, &Move(c1, c2, c3): &Move) -> Result<Game, (Game, PlayError)> {
    if (c1 == c2) || (c2 == c3) || (c1 == c3) || (c1 >= g.in_play.len()) || (c2 >= g.in_play.len()) || (c3 >= g.in_play.len()) {
        return Err((g, PlayError::InvalidMove));
    }

    let card1 = g.in_play[c1];
    let card2 = g.in_play[c2];
    let card3 = g.in_play[c3];

    if is_set(&card1, &card2, &card3) {
        g.score += 1;
        if g.deck.len() == 0 {
            // No more cards in deck. Remove cards from play and that's it
            sorted_move(&Move(c1, c2, c3)).iter().rev().for_each(|&i| {
                g.in_play.remove(i as usize);
            });
        } else if g.in_play.len() <= 12 {
            // Normal play. Replace cards in-place
            vec![c1, c2, c3].iter().for_each(|&i| {
                g.in_play[i] = g.deck.pop().expect("Impossible to fail to draw a card: deck always has multiple of 3 cards");
            });
        } else {
            // Too many cards. Do not replace
            sorted_move(&Move(c1, c2, c3)).iter().rev().for_each(|&i| {
                g.in_play.remove(i as usize);
            });
        }
        Ok(g)
    } else {
        g.score -= 1;
        Err((g, PlayError::NotASet))
    }
}

pub fn find_sets(g: &Game) -> Vec<Move> {
    let mut sets = vec![];

    for i in 0..g.in_play.len() {
        for j in (i+1)..g.in_play.len() {
            for k in (j+1)..g.in_play.len() {
                if is_set(&g.in_play[i], &g.in_play[j], &g.in_play[k]) {
                    sets.push(Move(i, j, k));
                }
            }
        }
    }

    sets
}

fn sorted_move(mve: &Move) -> Vec<usize> {
    let mut v = vec![mve.0, mve.1, mve.2];
    v.sort();
    v
}

fn make_deck() -> Vec<Card> {
    // Make the deck
    let mut deck = Vec::with_capacity(81);
    for number in vec![Number::One, Number::Two, Number::Three] {
        for shape in vec![Shape::Oval, Shape::Wave, Shape::Diamond] {
            for shading in vec![Shading::Empty, Shading::Half, Shading::Full] {
                for colour in vec![Colour::Red, Colour::Green, Colour::Purple] {
                    deck.push(Card {number, shape, shading, colour});
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
