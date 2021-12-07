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
        if let Some(solution) = Solver::new().solve(&map, 19) {
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
    rest_possibilities: usize,
    steps: usize,
    tried: HashMap<SolveState, Costs>,
    moves: Vec<Move>,
    solutions: Vec<Solution>,

    moves_search: Vec<Move>,
}

impl Solver {
    fn new() -> Self {
        Self {
            steps: 0,
            rest_possibilities: 0,
            tried: HashMap::new(),
            moves: Vec::new(),
            solutions: Vec::new(),
            moves_search: Vec::new(),
        }
    }

    fn solve(mut self, map: &Map, ttl: usize) -> Option<Solution> {
        self.internal_solve(map, ttl);
        println!(
            " ==> Stats: Steps={} RestPossibilities={}",
            self.steps, self.rest_possibilities
        );
        if self.solutions.is_empty() {
            None
        } else {
            println!("Found {} solutions", self.solutions.len());
            self.solutions.sort_by_key(|solution| solution.costs());
            self.solutions.into_iter().next()
        }
    }

    fn internal_solve(&mut self, map: &Map, ttl: usize) {
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
            self.solutions.push(Solution::new(self.moves.clone()));
            return;
        }

        if ttl == 0 && !debug {
            self.rest_possibilities += 1;
            return;
        }

        let current_costs = Costs::new(&self.moves);
        if let Some(costs) = self.tried.get(map.solve_state()) {
            if current_costs >= *costs {
                if debug {
                    println!("I was already here!");
                }
                return;
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
                self.internal_solve(&map, ttl - 1);
                self.moves.pop();
            }
        }
    }
}
