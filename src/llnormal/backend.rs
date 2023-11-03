use super::core::{GameCore, GameCard, is_set, GameMaker};
pub use super::core::{Number, Shape, Shading, Colour, Card};

pub struct Move(pub usize, pub usize, pub usize);

pub struct Game<'a> {
    core: GameCore<'a>,
    in_play: Vec<GameCard<'a>>,
}

impl<'a> Game<'a> {
    pub fn get_score(&self) -> i32 {
        self.core.get_score()
    }

    pub fn remaining_cards(&self) -> usize {
        self.core.remaining_cards()
    }

    pub fn cards_in_play(&self) -> &[GameCard<'a>] {
        &self.in_play
    }
    
    pub fn new((core, cards): (GameCore<'a>, [GameCard<'a>; 12])) -> Self {
        Game {
            core,
            in_play: cards.into(),
        }
    }

    pub fn attempt_move(&mut self, Move(c1, c2, c3): Move) -> Result<(), PlayError> {

        // Sort the card indices
        let mut v = vec![c1, c2, c3];
        v.sort();
        let [c1, c2, c3]: [usize; 3] = v.try_into().expect("Impossible to not have 3 values");
        
        if (c1 == c2) || (c2 == c3) || (c3 >= self.in_play.len()) {
            return Err(PlayError::InvalidMove);
        }

        let card3 = self.in_play.remove(c3);
        let card2 = self.in_play.remove(c2);
        let card1 = self.in_play.remove(c1);

        let return_cards = |v: &mut Vec<GameCard<'a>>, cards: [GameCard<'a>; 3]| {
            cards.into_iter()
                .zip([c1, c2, c3])
                .for_each(|(c, i)| {
                    v.insert(i.min(v.len()-1), c);
                });
        };



        match self.core.make_move([card1, card2, card3]) {
            Ok(None) => { // TODO: proper card moving with > 12 cards
                let a = [self.in_play.pop().unwrap(), self.in_play.pop().unwrap(), self.in_play.pop().unwrap()];
                return_cards(&mut self.in_play, a);
                Ok(())
            },        
            Ok(Some(cards)) => {
                return_cards(&mut self.in_play, cards);
                Ok(())
            }
            Err(cards) => {
                return_cards(&mut self.in_play, cards);
                Err(PlayError::NotASet)
            }
        }
        // if self.deck.len() == 0 {
        //     // No more cards in deck. Remove cards from play and that's it
        //     let mut v = vec![c1, c2, c3];
        //     v.sort();
        //     v.iter().rev().for_each(|&i| {
        //         self.in_play.remove(i as usize);
        //     });
        // } else if self.in_play.len() <= 12 {
        //     assert_eq!(self.in_play.len(), 12);
        //     // Normal play. Replace cards in-place
        //     vec![c1, c2, c3].iter().for_each(|&i| {
        //         self.in_play[i] = self.deck.pop().expect("Impossible to fail to draw a card: deck always has multiple of 3 cards");
        //     });
        
        // } else {
        //     // Too many cards. Do not replace

        //     // Remove cards and replace them with cards from the end (minimize card movements)
        //     // Here's how this is going to work:
        //     // 1. figure out where the end of the play area that's still going to exist is (current len - 3)
        //     // 2. remove all cards that are part of a set past that point
        //     // 3. Swap out the cards before the new end with those past the new end

        //     let new_max = self.in_play.len() - 3;

        //     let mut pre = vec![];
        //     let mut post = vec![];

        //     for i in [c1, c2, c3] {
        //         if i < new_max {
        //             pre.push(i);
        //         } else {
        //             post.push(i);
        //         }
        //     }

        //     // Sort post from largest to smallest (so that removals don't change positions of other cards to be removed)
        //     post.sort();
        //     post.reverse();

        //     for i in post {
        //         self.in_play.remove(i);
        //     }

        //     // Replace cards before the break with cards from the end
        //     for i in pre {
        //         self.in_play[i] = self.in_play.pop().expect("Deck cannot be empty");
        //     }

        //     assert_eq!(self.in_play.len() % 3, 0);
        // }

        // Ok(self)
    }

    pub fn draw_3(&mut self) -> Result<(), ()> {
        match self.core.draw_3() {
            None => Err(()),
            Some(cards) => {
                self.in_play.extend(cards.into_iter());
                Ok(())
            }
        }
    }

    pub fn find_sets(&self) -> Vec<Move> {
        // TODO: could maybe be more efficient with a set? But probably not just because of the overhead of a hashset lookup
        // Could probably be made more efficient with some bithacking nonsense (u128)
        let mut sets = vec![];

        for i in 0..self.in_play.len() {
            for j in (i+1)..self.in_play.len() {
                for k in (j+1)..self.in_play.len() {
                    if is_set(&self.in_play[i].get_card(), &self.in_play[j].get_card(), &self.in_play[k].get_card()) {
                        sets.push(Move(i, j, k));
                    }
                }
            }
        }

        sets
    }
}

pub enum PlayError {
    InvalidMove,
    NotASet,
}




pub fn has_set(_cards: &[Card]) -> bool {
    panic!("Not implemented!");
}

#[cfg(test)]
mod tests {
 
    use super::{Number::*, Shape::*, Shading::*, Colour::*};
    use super::*;
    
    #[test]
    fn test() {
 
    }
 }
