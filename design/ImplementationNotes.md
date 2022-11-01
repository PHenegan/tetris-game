### Model Implementation
10/29/2022
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

10/32/2022
- I'm realizing as I make this that I left out a pretty big amount of functionality in the model.
  - I had never accounted for any movement or rotation, since the tetromino should be able to move
    and rotate horizontally. 
  - Now I'm wondering how this should be designed, since there are two pretty different choices
    I could:
    - Move the tetromino into its own class instead of just using a 2D array.
      - PROS:
        - Simplifies the logic of the model, since it can just call the tetromino methods
        - might look neater
      - CONS:
        - The board and tetromino are kind of linked - what would happen in the case where
          the tetromino cannot be moved or rotated because something else is in the way?
        - The methods will have to be in the model trait anyway (because the controller has to be
          able to move/rotate the tetromino when the user inputs)
    - Continue using the tetromino the way I am, but also have logic for moving and rotating with
      the appropriate methods.