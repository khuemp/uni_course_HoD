use cleanav::{markov::{flatten_coordinate, unflatten_coordinate, possible_moves, compute_step, StochasticModel}, analysis::generate_transition_matrix};
use nalgebra::{vector, Vector};
use assert_approx_eq::assert_approx_eq;

#[macro_export]
macro_rules! nalgebra_assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let eps = 1.0e-6;
        let (a, b) = (&$a, &$b);
        assert!(
            (a - b).abs() < eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (a - b).abs()
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            (a - b).abs() < eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (a - b).abs()
        );
    }};
}


#[test]
fn test_coordinate_unflatten() {
    let index = 35;
    let width = 10;
    assert_eq!(unflatten_coordinate(index, width), (5, 3));
}

#[test]
fn test_coordinate_flatten() {
    assert_eq!(flatten_coordinate((5, 3), 10), 35);
}

#[test]
fn test_moves() {
    let moves = possible_moves();
    assert_eq!(moves.len(), 1+8+8);
    assert!(moves.contains(&(0,0)));
    assert!(moves.contains(&(-1,1)));
    assert!(moves.contains(&(2,0)));
}

#[test]
fn test_step_wrap() {
    assert_eq!(compute_step((0,0), (-1, -1), (3,3)), (2,2))
}

#[test]
fn test_matrix_size() {
    let matrix = generate_transition_matrix(5, 3);
    let cell_count = 5*3;
    assert_eq!(matrix.shape(), (cell_count, cell_count));
}

#[test]
fn test_is_small_matrix_stochastic() {
    let matrix = generate_transition_matrix(2, 2);
    nalgebra_assert_approx_eq!(matrix.column_sum(), vector![1.0, 1.0, 1.0, 1.0], vector![0.01, 0.01, 0.01, 0.01]);
}

#[test]
fn test_is_larger_matrix_stochastic() {
    let matrix = generate_transition_matrix(5, 4);
    nalgebra_assert_approx_eq!(matrix.column_sum(), Vector::from([1.0; 20]), Vector::from([0.01; 20]));
}

#[test]
fn test_possible_transition() {
    // two steps NW
    let matrix = generate_transition_matrix(5, 4);
    let a = (3,3);
    let b = (1,1);
    assert!(matrix[(flatten_coordinate(a, 5), flatten_coordinate(b, 5))] > 0.0);
}

#[test]
fn test_impossible_transition() {
    // two steps N, one step W
    let matrix = generate_transition_matrix(5, 4);
    let a = (3,3);
    let b = (2,1);
    assert_eq!(matrix[(flatten_coordinate(a, 5), flatten_coordinate(b, 5))], 0.0);
}

#[test]
fn test_no_step_probability() {
    let m = StochasticModel::new(3, 5);
    let steps = 0;
    assert_approx_eq!(m.compute_transition_probability((1, 1), (1, 1), steps), 1.0, 0.01);
    assert_approx_eq!(m.compute_transition_probability((1, 1), (1, 2), steps), 0.0, 0.01);
}

#[test]
fn test_two_steps_probability() {
    let m = StochasticModel::new(10, 10);
    let steps = 2;
    assert!(m.compute_transition_probability((0, 0), (0, 0), steps) > 0.0);
    assert!(m.compute_transition_probability((0, 0), (1, 0), steps) > 0.0);
    assert!(m.compute_transition_probability((0, 0), (2, 0), steps) > 0.0);
    assert!(m.compute_transition_probability((0, 0), (3, 0), steps) > 0.0);
    assert!(m.compute_transition_probability((0, 0), (4, 0), steps) > 0.0);
    assert!(m.compute_transition_probability((0, 0), (5, 0), steps) == 0.0); // unreachable
    assert!(m.compute_transition_probability((0, 0), (6, 0), steps) > 0.0); // due to wrap around
}

#[test]
fn test_long_random_walk() {
    // after enough steps all cells should have roughly the same probability
    let m = StochasticModel::new(3, 3);
    let steps = 200;
    let expected = 1. / 9.;
    let epsilon = 1e-7;
    assert_approx_eq!(m.compute_transition_probability((1, 1), (1, 1), steps), expected, epsilon);
    assert_approx_eq!(m.compute_transition_probability((1, 1), (0, 0), steps), expected, epsilon);
}

#[test]
fn test_manhattan_distance() {
    let m = StochasticModel::new(8, 8);
    assert_eq!(m.manhattan_distance((0, 2), (0, 7)), 3.0);
    assert_eq!(m.manhattan_distance((6, 3), (6, 5)), 2.0);
    assert_eq!(m.manhattan_distance((1, 1), (7, 7)), 4.0);
    assert_eq!(m.manhattan_distance((2, 2), (7, 7)), 6.0);
    assert_eq!(m.manhattan_distance((5, 2), (3, 4)), 4.0);
    assert_eq!(m.manhattan_distance((7, 7), (0, 0)), 2.0);
}

#[test]
fn test_no_step_distance() {
    let m = StochasticModel::new(5, 5);
    assert_eq!(m.expected_distance((1, 1), 0), 0.0);
}

#[test]
fn test_one_step_distance() {
    let m = StochasticModel::new(5, 5);
    let dist = m.expected_distance((1, 1), 1);
    assert_approx_eq!(dist, 2.1176470588235294, 0.01);
    let dist = m.expected_distance((3, 3), 1);
    assert_approx_eq!(dist, 2.1176470588235294, 0.01);
}

#[test]
fn test_multi_step_distance() {
    let m = StochasticModel::new(7, 7);
    let dist = m.expected_distance((1, 1), 100);
    assert_approx_eq!(dist, 3.428571428571427, 0.01);
    let dist = m.expected_distance((4, 4), 100);
    assert_approx_eq!(dist, 3.428571428571427, 0.01);
    let dist = m.expected_distance((1, 6), 100);
    assert_approx_eq!(dist, 3.428571428571427, 0.01);
}
