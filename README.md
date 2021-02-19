# RustProject
Nine men's morris, played through the command-line interface.

More detail on rules: https://en.wikipedia.org/wiki/Nine_men%27s_morris

During the initial phase, players are asked to input the coordinates of a position on which to place their piece, e.g. "a7".
During the second phase, players are asked to input the coordinates of two positions, in order to move a piece from the first to the second, e.g. "a7a4".
If a player forms a mill, they are also asked to input the coordinates of a position occupied by their opponent, in order to remove a piece from it.

On every turn, the board is printed to the console with the positions marked as either empty (·), occupied by a white piece (○), or occupied by a black piece (●) (note that on a black terminal background, the symbols can appear inversed - the white piece is a circle filled in with black, whereas the black piece is a circle filled in with white).
