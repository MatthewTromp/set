use std::marker::PhantomData;
use derivative::Derivative;

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

#[derive(Derivative)]
#[derivative(Debug)]
pub struct GameCard<'a> {
    card: Card,
    #[derivative(Debug="ignore")]
    phantom: PhantomData<&'a mut &'a ()>,
}

impl<'a> GameCard<'a> {
    pub fn get_card(&self) -> &Card {
        &self.card
    }
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

    pub fn to_int(&self) -> u8 {
        let base: u8 = 3;
        (match &self.number {
            Number::One => 0,
            Number::Two => base.pow(3),
            Number::Three => 2*base.pow(3) as u8,
        } + match &self.shape {
            Shape::Oval => 0,
            Shape::Wave => base.pow(2),
            Shape::Diamond => 2*base.pow(2),
        } + match &self.shading {
            Shading::Empty => 0,
            Shading::Half => 3,
            Shading::Full => 3*2,
        } + match &self.colour {
            Colour::Red => 0,
            Colour::Green => 1,
            Colour::Purple => 2,
        })
    }
}

pub struct GameMaker;

impl GameMaker {
    pub fn with_core<'a, F: for <'any> FnOnce((GameCore<'any>, [GameCard<'any>; 12]))>(&self, func: F) {
        func(
            GameCore::new(make_shuffled_deck())
        )
    }

    fn with_unshuffled_core<'a, F: for <'any> FnOnce((GameCore<'any>, [GameCard<'any>; 12]))>(&self, func: F) {
        func(
            GameCore::new(make_unshuffled_deck())
        )
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct GameCore<'a> {
    deck: Vec<Card>,
    extant_cards: usize,
    score: i32,
    #[derivative(Debug="ignore")]
    mark_invariant: PhantomData<&'a mut &'a ()>,
}

impl<'a> GameCore<'a> {
    fn new(deck: Vec<Card>) -> (Self, [GameCard<'a>; 12]) {
        let mut game = GameCore {
            deck: vec![],
            extant_cards: 12,
            score: 0,
            mark_invariant: PhantomData,
        };
        
        let mut iter = deck.into_iter();

        // Draw 12 cards
        let cards: [GameCard<'a>; 12] = (0..12)
            .map(|_| {iter.next().unwrap()})
            .map(|c| {GameCard {card: c, phantom: PhantomData}})
            .collect::<Vec<GameCard<'a>>>()
            .try_into()
            .expect("Cannot fail to get 12 cards from freshly made deck");

        game.deck = iter.collect();

        (game, cards)
    }
    
    pub fn remaining_cards(&self) -> usize {
        self.deck.len()
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn draw_3(&mut self) -> Option<[GameCard<'a>; 3]> {
        let out = self.draw_3_internal();
        
        if out.is_some() {
            self.score -= 1;
        }

        out
    }

    fn draw_3_internal(&mut self) -> Option<[GameCard<'a>; 3]> {
        let out = (0..3).map(|_| {
            match self.deck.pop() {
                Some(c) => Some(GameCard{card: c, phantom: PhantomData}),
                None => None,
            }
        }).collect::<Option<Vec<GameCard<'a>>>>()
            .map(|v| {v.try_into().expect("Must have taken 3 cards by construction")});

        if out.is_some() {
            self.extant_cards += 3;
        }

        out
    }

    pub fn make_move(&mut self, cards: [GameCard<'a>; 3]) -> Result<Option<[GameCard<'a>; 3]>, [GameCard<'a>; 3]> {
        
        if is_set(&cards[0].card, &cards[1].card, &cards[2].card) {
            self.score += 1;
            self.extant_cards -= 3;
            if self.extant_cards < 12 {
                Ok(self.draw_3_internal())
            } else {
                Ok(None)
            }
        } else {
            self.score -= 1;
            Err(cards)
        }
    }

    
}

fn make_unshuffled_deck() -> Vec<Card> {
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
    let mut deck = make_unshuffled_deck();

    // Shuffle the deck
    deck.shuffle(&mut thread_rng());

    deck
}

pub fn is_set(card1: &Card, card2: &Card, card3: &Card) -> bool {
    (
            (card1 != card2) // If two cards are the same, all three cards have to be the same for a set
    ) && (
	    (card1.number == card2.number && card2.number == card3.number) || (card1.number != card2.number && card2.number != card3.number && card3.number != card1.number)
    ) && (
	    (card1.shape == card2.shape && card2.shape == card3.shape) || (card1.shape != card2.shape && card2.shape != card3.shape && card3.shape != card1.shape)
    ) && (
	    (card1.shading == card2.shading && card2.shading == card3.shading) || (card1.shading != card2.shading && card2.shading != card3.shading && card3.shading != card1.shading)
    ) && (
	    (card1.colour == card2.colour && card2.colour == card3.colour) || (card1.colour != card2.colour && card2.colour != card3.colour && card3.colour != card1.colour)
    )
}

#[cfg(test)]
mod tests {
    use super::{*, Number::*, Shape::*, Colour::*, Shading::*};
    
    #[test]
    fn test_make_deck() {
        let deck = make_unshuffled_deck();
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
    fn test_lifetime_isolation_1() {
        let game_maker1 = GameMaker;
        game_maker1.with_core(|(mut core1, mut cards1)| {
            let game_maker2 = GameMaker;
            game_maker2.with_core(|(mut core2, mut cards2)| {
                let mut cards1 = Vec::from(cards1);
                core1.make_move([cards1.pop().unwrap(), cards1.pop().unwrap(), cards1.pop().unwrap()]);
                let mut cards2 = Vec::from(cards2);
                core2.make_move([cards2.pop().unwrap(), cards2.pop().unwrap(), cards2.pop().unwrap()]);
                // SHOULD NOT COMPILE WITH EITHER OF THE FOLLOWING LINES
                //core1.make_move([cards2.pop().unwrap(), cards2.pop().unwrap(), cards2.pop().unwrap()]);
                //core2.make_move([cards1.pop().unwrap(), cards1.pop().unwrap(), cards1.pop().unwrap()]);
            })
        })
    }

    #[test]
    fn test_lifetime_isolation_2() {
        let game_maker1 = GameMaker;
        let game_maker2 = GameMaker;
        game_maker1.with_core(|(mut core1, mut cards1)| {
            game_maker2.with_core(|(mut core2, mut cards2)| {
                let mut cards1 = Vec::from(cards1);
                core1.make_move([cards1.pop().unwrap(), cards1.pop().unwrap(), cards1.pop().unwrap()]);
                let mut cards2 = Vec::from(cards2);
                core2.make_move([cards2.pop().unwrap(), cards2.pop().unwrap(), cards2.pop().unwrap()]);
                // SHOULD NOT COMPILE WITH EITHER OF THE FOLLOWING LINES
                //core1.make_move([cards2.pop().unwrap(), cards2.pop().unwrap(), cards2.pop().unwrap()]);
                //core2.make_move([cards1.pop().unwrap(), cards1.pop().unwrap(), cards1.pop().unwrap()]);
            })
        })
    }

    #[test]
    fn test_lifetime_isolation_3() {
        let game_maker1 = GameMaker;
        game_maker1.with_core(|(mut core1, mut cards1)| {
            game_maker1.with_core(|(mut core2, mut cards2)| {
                let mut cards1 = Vec::from(cards1);
                core1.make_move([cards1.pop().unwrap(), cards1.pop().unwrap(), cards1.pop().unwrap()]);
                let mut cards2 = Vec::from(cards2);
                core2.make_move([cards2.pop().unwrap(), cards2.pop().unwrap(), cards2.pop().unwrap()]);
                // SHOULD NOT COMPILE WITH EITHER OF THE FOLLOWING LINES
                //core1.make_move([cards2.pop().unwrap(), cards2.pop().unwrap(), cards2.pop().unwrap()]);
                //core2.make_move([cards1.pop().unwrap(), cards1.pop().unwrap(), cards1.pop().unwrap()]);
            })
        })
    }
}
