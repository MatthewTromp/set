# Set (in rust!)

This is an implementation of the game Set (and its cousin, Projective Set) written in rust, that you can play in your terminal

## Usage

To play set, run the executable.
To play projective set, pass the argument `--projective`.
To run a version of Set written with linear logic
(where the frontend actually handles cards that use lifetime shenanigans to ensure you can't duplicate cards or use cards from different games together),
pass the argument `--linear`.

Note that the linear version is the same as the normal version
(although it lacks some features like the ability to quit,
though why you would want to quit such a mathematically perfect implementation as this is beyond me).
The only difference is that you get to know the game you're playing is implemented in a very obtuse way.

## How to play

For normal Set,
use the letter keys on your to select 3 cards forming a set,
and press enter.
The positions of the keys correspond to the locations of the cards on screen.
So the three leftmost cards correspond to the letters qaz,
the next three cards correspond to the letters wsx,
and so on.

If you cannot find a set,
press p to draw three more cards
(note that this costs you a point).

You can quit with k,
and ; will tell you what sets are currently on the board
(but don't use that, it's cheating)

For Projective Set, each card is a line of coloured squares.
The lines are numbered,
so just enter numbers corresponding to a valid set.
This is the version of Projective Set where you can enter as many numbers as you want.
Since there is always a valid set on the board in this version of projective set,
there is no way to draw more cards.