# Conway's Game of Life

This project is an implementation of Conway's Game of Life using the `ggez` game framework in Rust, created as part of my learning journey with Rust.

## Overview

Conway's Game of Life is a cellular automaton devised by the British mathematician John Horton Conway in 1970. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. One interacts with the Game of Life by creating an initial configuration and observing how it evolves.

## How to Run

To run this project, you need to have Rust and Cargo installed. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/).

1. Clone the repository:
    ```sh
    git clone https://github.com/ridolud/rd-game-of-Life.git
    cd game-of-life
    ```

2. Build and run the project:
    ```sh
    cargo run
    ```

## Controls

- **Up Arrow**: Increase the simulation speed.
- **Down Arrow**: Decrease the simulation speed.
- **Delete**: Reset the grid.
- **Mouse Click**: Pause the simulation and toggle cells on the grid.