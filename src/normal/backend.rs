use rand::thread_rng;
use rand::seq::SliceRandom;

enum AbstractOption {
    A,
    B,
    C,
}

fn opt_completion(a1: AbstractOption, a2: AbstractOption) -> AbstractOption {
    use AbstractOption::*;
    match (a1, a2) {
        (A, A) => A,
        (A, B) => C,
        (A, C) => B,
        (B, A) => C,
        (B, B) => B,
        (B, C) => A,
        (C, A) => B,
        (C, B) => A,
        (C, C) => C,
    }   
}
    

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Number {
    One,
    Two,
    Three,
}


impl Number {
    fn to_abstract_option(&self) -> AbstractOption {
        use Number::*;
        use AbstractOption::*;
        match self {
            One => A,
            Two => B,
            Three => C,
        }
    }

    fn from_abstract_option(a: AbstractOption) -> Self {
        use Number::*;
        use AbstractOption::*;
        match a {
            A => One,
            B => Two,
            C => Three,
        }
    }

    fn to_complete(v1: &Number, v2: &Number) -> Number {
        Self::from_abstract_option(opt_completion(v1.to_abstract_option(), v2.to_abstract_option()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shape {
    Oval,
    Wave,
    Diamond,
}

impl Shape {
    fn to_abstract_option(&self) -> AbstractOption {
        use Shape::*;
        use AbstractOption::*;
        match self {
            Oval => A,
            Wave => B,
            Diamond => C,
        }
    }

    fn from_abstract_option(a: AbstractOption) -> Self {
        use Shape::*;
        use AbstractOption::*;
        match a {
            A => Oval,
            B => Wave,
            C => Diamond,
        }
    }

    fn to_complete(v1: &Shape, v2: &Shape) -> Shape {
        Self::from_abstract_option(opt_completion(v1.to_abstract_option(), v2.to_abstract_option()))
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shading {
    Empty,
    Half,
    Full,
}

impl Shading {
    fn to_abstract_option(&self) -> AbstractOption {
        use Shading::*;
        use AbstractOption::*;
        match self {
            Empty => A,
            Half => B,
            Full => C,
        }
    }

    fn from_abstract_option(a: AbstractOption) -> Self {
        use Shading::*;
        use AbstractOption::*;
        match a {
            A => Empty,
            B => Half,
            C => Full,
        }
    }

    fn to_complete(v1: &Shading, v2: &Shading) -> Shading {
        Self::from_abstract_option(opt_completion(v1.to_abstract_option(), v2.to_abstract_option()))
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Colour {
    Red,
    Green,
    Purple,
}

impl Colour {
    fn to_abstract_option(&self) -> AbstractOption {
        use Colour::*;
        use AbstractOption::*;
        match self {
            Red => A,
            Green => B,
            Purple => C,
        }
    }

    fn from_abstract_option(a: AbstractOption) -> Self {
        use Colour::*;
        use AbstractOption::*;
        match a {
            A => Red,
            B => Green,
            C => Purple,
        }
    }

    fn to_complete(v1: &Colour, v2: &Colour) -> Colour {
        Self::from_abstract_option(opt_completion(v1.to_abstract_option(), v2.to_abstract_option()))
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    number: Number,
    shape: Shape,
    shading: Shading,
    colour: Colour,
}

// Returns the card that would complete a set with the other two cards
pub fn third_card(c1: &Card, c2: &Card) -> Card {
    Card::new(
        Number::to_complete(&c1.number, &c2.number),
        Shape::to_complete(&c1.shape, &c2.shape),
        Shading::to_complete(&c1.shading, &c2.shading),
        Colour::to_complete(&c1.colour, &c2.colour))
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

pub struct Move(pub usize, pub usize, pub usize);

impl Game {
    pub fn remaining_cards(&self) -> usize {
        self.deck.len()
    }

    pub fn cards_in_play(&self) -> &[Card] {
        &self.in_play[..]
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    // draw 3 new cards for when the user can't find any more cards
    pub fn draw_3(&mut self) -> Result<(), ()> {
        for _ in 0..3 {
            match self.deck.pop() {
                Some(c) => self.in_play.push(c),
                None => return Err(()),
            }
        }

        // Deduct a point from the score
        self.score -= 1;

        Ok(())
    }

    pub fn new() -> Self {
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

    pub fn attempt_move(&mut self, &Move(c1, c2, c3): &Move) -> Result<(), PlayError> {
        if (c1 == c2) || (c2 == c3) || (c1 == c3) || (c1 >= self.in_play.len()) || (c2 >= self.in_play.len()) || (c3 >= self.in_play.len()) {
            return Err(PlayError::InvalidMove);
        }

        let card1 = self.in_play[c1];
        let card2 = self.in_play[c2];
        let card3 = self.in_play[c3];

        if !is_set(&card1, &card2, &card3) {
            self.score -= 1;
            return Err(PlayError::NotASet);
        }
        
        self.score += 1;
        if self.deck.len() == 0 {
            // No more cards in deck. Remove cards from play and that's it
            sorted_move(&Move(c1, c2, c3)).iter().rev().for_each(|&i| {
                self.in_play.remove(i as usize);
            });
        } else if self.in_play.len() <= 12 {
            assert_eq!(self.in_play.len(), 12);
            // Normal play. Replace cards in-place
            vec![c1, c2, c3].iter().for_each(|&i| {
                self.in_play[i] = self.deck.pop().expect("Impossible to fail to draw a card: deck always has multiple of 3 cards");
            });
        } else {
            // Too many cards. Do not replace

            // Remove cards and replace them with cards from the end (minimize card movements)
            // Here's how this is going to work:
            // 1. figure out where the end of the play area that's still going to exist is (current len - 3)
            // 2. remove all cards that are part of a set past that point
            // 3. Swap out the cards before the new end with those past the new end

            let new_max = self.in_play.len() - 3;

            // Indices to remove that are before the new end
            let mut pre = vec![];
            // Indices to remove that are after the new end
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
                self.in_play.remove(i);
            }

            // Replace cards before the break with cards from the end
            for i in pre {
                self.in_play[i] = self.in_play.pop().expect("Deck cannot be empty");
            }

            assert_eq!(self.in_play.len() % 3, 0);
        }

        Ok(())
    }

}





pub enum PlayError {
    InvalidMove,
    NotASet,
}


pub fn find_sets(g: &Game) -> Vec<Move> {
    // TODO: could maybe be more efficient with a set? But probably not just because of the overhead of a hashset lookup
    // Could probably be made more efficient with some bithacking nonsense (u128)
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

pub fn make_deck() -> Vec<Card> {
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

pub fn make_shuffled_deck() -> Vec<Card> {
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
    fn third_card_test() {
        let deck = make_deck();
        for c1 in &deck {
            for c2 in &deck {
                if c1 == c2 {
                    continue;
                }
                let target_c3 = third_card(c1, c2);
                for c3 in &deck {
                    assert_eq!(c3 == &target_c3, is_set(c1, c2, c3))
                }
            }
        }
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
