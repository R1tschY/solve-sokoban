use std::str::FromStr;
use solve_sokoban::{Costs, Input, Map};
use solve_sokoban::solver::Solver;

#[test]
fn test_ttac_2021_1() {
    let input = Input::from_str(include_str!("ttac2021/level1.txt"));
    let map = Map::from(input.unwrap());
    if let Some(solution) = Solver::new().solve(&map, 20) {
        assert_eq!(Costs::new(12, 26), solution.costs());
    }
}

#[test]
fn test_ttac_2021_2() {
    let input = Input::from_str(include_str!("ttac2021/level2.txt"));
    let map = Map::from(input.unwrap());
    if let Some(solution) = Solver::new().solve(&map, 20) {
        assert_eq!(Costs::new(12, 54), solution.costs());
    }
}

#[test]
fn test_ttac_2021_3() {
    let input = Input::from_str(include_str!("ttac2021/level3.txt"));
    let map = Map::from(input.unwrap());
    if let Some(solution) = Solver::new().solve(&map, 20) {
        assert_eq!(Costs::new(13, 48), solution.costs());
    }
}