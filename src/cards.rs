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

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub number: Number,
    pub shape: Shape,
    pub shading: Shading,
    pub colour: Colour,
}

pub fn is_set(card1: &Card, card2: &Card, card3: &Card) -> bool {
    return (
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
