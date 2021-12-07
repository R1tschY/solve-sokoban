use solve_sokoban::{Costs, Input, Map, Move, Pos, Solution, SolveState};
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = Input::from_str(include_str!("levels/level1.txt"));
    let map = Map::from(input.unwrap());

    if false {
        let search = vec![
            Move::new(Pos::new(2, 4), Pos::new(2, 5)),
            Move::new(Pos::new(2, 5), Pos::new(2, 6)),
            //
            Move::new(Pos::new(5, 3), Pos::new(4, 3)),
            Move::new(Pos::new(4, 3), Pos::new(3, 3)),
        ];
        let move_count = search.len();

        let mut solver = Solver::new();
        solver.moves_search = search;
        solver.solve(&map, move_count);

        println!(
            "{:?}",
            Move {
                start: Pos { x: 4, y: 3 },
                end: Pos { x: 3, y: 3 }
            } == Move::new(Pos::new(4, 3), Pos::new(3, 3))
        );
    } else {
        if let Some(moves) = Solver::new().solve(&map, 13) {
            let solution = Solution::new(moves);

            println!("{:?}", solution.costs());

            let mut map = map.clone();
            for (i, mv) in solution.moves().iter().enumerate() {
                map.apply_move(*mv);

                println!();
                println!("Step {}:", i + 1);
                println!("{}", map);
            }
        }
    }
}

struct Solver {
    solutions: usize,
    steps: usize,
    tried: HashMap<SolveState, Costs>,
    moves: Vec<Move>,

    moves_search: Vec<Move>,
}

impl Solver {
    fn new() -> Self {
        Self {
            steps: 0,
            solutions: 0,
            tried: HashMap::new(),
            moves: Vec::new(),
            moves_search: Vec::new(),
        }
    }

    fn solve(mut self, map: &Map, ttl: usize) -> Option<Vec<Move>> {
        let moves = self.internal_solve(map, ttl);
        println!(
            " ==> Stats: Steps={} RestPossibilities={}",
            self.steps, self.solutions
        );
        moves
    }

    fn internal_solve(&mut self, map: &Map, ttl: usize) -> Option<Vec<Move>> {
        let debug = if !self.moves_search.is_empty() && self.moves_search == self.moves {
            println!("State!\n{}", map);
            true
        } else {
            false
        };

        if map.is_solved() {
            if debug {
                println!("Solved!");
            }
            return Some(vec![]);
        }

        if ttl == 0 && !debug {
            self.solutions += 1;
            return None;
        }

        let current_costs = Costs::new(&self.moves);
        if let Some(costs) = self.tried.get(map.solve_state()) {
            if current_costs >= *costs {
                if debug {
                    println!("I was already here!");
                }
                return None;
            }
            if debug {
                println!("I have less costs!");
            }
        }
        self.tried.insert(map.solve_state().clone(), current_costs);

        let moves = map.possible_moves();
        for m in moves.iter() {
            let mut map = map.clone();
            map.apply_move(*m);

            if debug {
                println!();
                println!("Possible Move:");
                println!("{}", map);
            } else {
                self.steps += 1;
                self.moves.push(*m);
                if let Some(mut moves) = self.internal_solve(&map, ttl - 1) {
                    moves.insert(0, m.clone());
                    return Some(moves);
                }
                self.moves.pop();
            }
        }

        None
    }
}
