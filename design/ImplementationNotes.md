### Model Implementation
- I decided for now on just doing an "update" method for now. I might change this approach later
  once I see how it fits together with a controller to play the game.
  - The update method just moves the tetromino down and maybe places it if necessary.
- originally, I wanted to be able to check if the game was over from the model, but now I am thinking
  this might actually be the task of the controller in this situation. After thinking about it,
  I realized that checking if the game is over would mean checking if a specific tetromino could
  be spawned at the top of the grid. Maybe replacing a game_over check with a "can_spawn" check
  which takes in a tetromino and sees if it can be placed at the given coordinates might be better.
  - Again, this might be something I think about once I have begun looking at the controller, so I can
    see how this fits together.