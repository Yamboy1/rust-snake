# Snake game process

- Setup board, snake, direction and initial food
- Every loop
  - Get user input
  - If snake head is on food tile then move and grow, else just move
  - If snake is touching itself then end
  - If snake touching walls then end
  - wait 500ms