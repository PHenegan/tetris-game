This is just some of my initial thoughts on design. This is intended to be kind of like a design journal so I can keep track the reasoning behind my approach


# Requirements for my idea:

### Things I will definitely need to implement:
* create a grid of cells: this would be the tetris board
* remove a row of cells if it is completely full
* create the tetris pieces
  * each piece will spawn at the top of the board and fall downwards, so the cells in the piece have to be able to fall together (i.e. if one cell in the piece can't fall the rest shouldn't be able to either)
  * rotate the piece - should be able to rotate the shape of the piece left and right
* the row should not be cleared until the piece "lands", meaning it can't move down any more. After the piece lands, the next piece should be spawned at the top
* the game ends if part of a piece lands above 

### Nice to have, but not a priority
* Being able to store one tetris piece and swap it with the one currently falling
* Give the player some time before "placing" the tetris piece as it lands, so they can rotate/move it
