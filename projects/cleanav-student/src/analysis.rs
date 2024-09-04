use nalgebra::{min, DMatrix, RowDVector};

use crate::markov::{compute_step, flatten_coordinate, possible_moves, unflatten_coordinate};

pub fn generate_transition_matrix(width: usize, height: usize) -> DMatrix<f64> {
    // Hint: use possible_moves() and compute_step() from crate::markov
    let mut transition_matrix = DMatrix::from_element(width * height, width * height, 0.0);

    for (r, mut row) in transition_matrix.row_iter_mut().enumerate() {
        for (d_x, d_y) in possible_moves().into_iter() {
            let (new_x, new_y) =
                compute_step(unflatten_coordinate(r, width), (d_x, d_y), (width, height));
            row[flatten_coordinate((new_x, new_y), width)] += 1.0 / (possible_moves().len() as f64);
        }
    }

    transition_matrix
}

impl crate::markov::StochasticModel {
    // Take a look at what is already implemented in this struct ( see `markov.rs` )

    pub fn compute_state_distribution(
        &self,
        start: (usize, usize),
        steps: usize,
    ) -> RowDVector<f64> {
        // Hint: s_{t+1} = s_t * P
        let mut s = RowDVector::from_element(self.width * self.height, 0.0);
        s[flatten_coordinate(start, self.width)] = 1.0;

        for _ in 0..steps {
            s = s * self.transition_matrix.clone();
        }

        s
    }

    pub fn compute_transition_probability(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        steps: usize,
    ) -> f64 {
        // Hint: use compute_state_distribution
        self.compute_state_distribution(start, steps)[flatten_coordinate(end, self.width)]
    }

    pub fn manhattan_distance(&self, from: (usize, usize), to: (usize, usize)) -> f64 {
        // Hint: The obvious path might not be the fastest (keep position wrapping in mind)
        let dist_x = min(
            (from.0).abs_diff(to.0),
            self.width - (from.0).abs_diff(to.0),
        );
        let dist_y = min(
            (from.1).abs_diff(to.1),
            self.height - (from.1).abs_diff(to.1),
        );
        (dist_x + dist_y) as f64
    }

    pub fn expected_distance(&self, start: (usize, usize), steps: usize) -> f64 {
        // Hint: use manhattan_distance
        let mut sum = 0.0;
        for y in 0..self.height {
            for x in 0..self.width {
                sum += self.compute_transition_probability(start, (x, y), steps)
                    * self.manhattan_distance(start, (x, y))
            }
        }
        sum
    }
}
