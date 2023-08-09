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

use rand::thread_rng;
use rand::seq::SliceRandom;

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

#[derive(Clone, Copy, Debug)]
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

pub fn make_deck() -> Vec<Card> {
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

pub fn make_shuffled_deck() -> Vec<Card> {
    let mut deck = make_deck();

    // Shuffle the deck
    deck.shuffle(&mut thread_rng());

    deck
}

    

pub fn is_set(card1: &Card, card2: &Card, card3: &Card) -> bool {
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
	assert!(deck.
}
