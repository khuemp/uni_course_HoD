use std::iter;

use nalgebra::DMatrix;

use crate::analysis::generate_transition_matrix;

pub struct StochasticModel {
    pub transition_matrix: DMatrix<f64>,
    pub width: usize,
    pub height: usize,
}

impl StochasticModel {
    pub fn new(width: usize, height: usize) -> Self {
        StochasticModel {
            width,
            height,
            transition_matrix: generate_transition_matrix(width, height),
        }
    }

    pub fn to_pos(&self, index: usize) -> (usize, usize) {
        unflatten_coordinate(index, self.width)
    }

    pub fn to_index(&self, pos: (usize, usize)) -> usize {
        flatten_coordinate(pos, self.width)
    }
}

pub fn flatten_coordinate(position: (usize, usize), width: usize) -> usize {
    // coordinate pair to index
    position.1 * width + position.0
}

pub fn unflatten_coordinate(index: usize, width: usize) -> (usize, usize) {
    // map index to coordinate pair
    (index % width, index / width)
}

pub fn possible_moves() -> Vec<(i32, i32)> {
    // Possible directions (N, NE, E, SE, S, SW, W, NW)
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    // Combine all directions with strengths 0, 1, 2
    let moves = iter::once((0, 0))
        .chain(directions)
        .chain(directions.iter().map(|(x, y)| (2 * x, 2 * y)));

    moves.collect()
}

pub fn compute_step(
    (i, j): (usize, usize),
    (dx, dy): (i32, i32),
    (width, height): (usize, usize),
) -> (usize, usize) {
    // New position is wrapped across the grid
    // New position after being pushed by the wind in direction (dx, dy)
    // (0+(-2)).rem_euclid(5) = 3 while (0+(-2)) mod (5) = -2
    let new_i = (i as i32 + dx).rem_euclid(width as i32) as usize;
    let new_j = (j as i32 + dy).rem_euclid(height as i32) as usize;
    (new_i, new_j)
}
