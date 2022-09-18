# Model
<p>
  Deciding the distinction between the model and controller here is actually surprisingly difficult. For example, what if I wanted to add a power-up system?
  This would be a pretty cool optional feature to have. However, it raises questions about how power-ups would be applied. Take the following 2 examples:
  
* An anti-gravity powerup, which temporarily reverses the gravity of the board
* A score-doubling powerup, which doubles the amount added to the score each time points are added.
  * Maybe setScore() and getScore() could be used? though this seams like it's exposing too many implementation details
  * Maybe use some sort of decorator pattern with mutation methods being protected so they can only be used there?
  * Maybe include methods in the model which mutate the modifiers (e.g. gravity, score multiplier, etc.)
    * Maybe this could be done with a hash table (e.g. setModifier(String modName, double/int value))
</p>

### Tetris Grid
- place a block (group of cells) down somewhere - throw exception if not possible
- clear a row (removes everything in that row) - throw exception if row is not empty
- check if a row is full
- check if a given cell is empty or not
- get the score of the game

### Cell
- probably its own class/interface
- has some form of type: maybe distinguishing the type of block it came from:
  - stair_left, stair_right, t, block, line, L_left, L_right
  - However, this tightly links the cell with the number of different block shapes. Is this fine or not? Tetris games typically don't involve different shapes,
but is it unreasonable to expect that?


## Language Notes
- I originally intended to use Java with either Swing or JavaFX, but I think it would be an interesting opportunity for me to instead do this with Rust and GTK. Some things won't translate as well, but interfaces should be able to become "traits" and implementations of those interfaces could become structs.
- Overall, even though Rust is a little bit less object oriented, I still think that doing it this way might be a good learning experience.
