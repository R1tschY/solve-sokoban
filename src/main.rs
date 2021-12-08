use solve_sokoban::{Costs, Input, Map, Move, Pos, Solution, SolveState};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn main() {
    let input = Input::from_str(include_str!("levels/level2.txt"));
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
        solver.solve(&map, move_count + 1);

        println!(
            "{:?}",
            Move {
                start: Pos { x: 4, y: 3 },
                end: Pos { x: 3, y: 3 }
            } == Move::new(Pos::new(4, 3), Pos::new(3, 3))
        );
    } else {
        if let Some(solution) = Solver::new().solve(&map, 20) {
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
    duration: Option<Duration>,
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
            duration: None,
            steps: 0,
            rest_possibilities: 0,
            tried: HashMap::new(),
            moves: Vec::new(),
            solutions: Vec::new(),
            moves_search: Vec::new(),
        }
    }

    fn solve(mut self, map: &Map, ttl: usize) -> Option<Solution> {
        let start = Instant::now();
        self.solve_iterative(map, ttl);
        self.duration = Some(Instant::now().duration_since(start));
        println!(
            " ==> Stats: Steps={} RestPossibilities={} Duration={:?}",
            self.steps, self.rest_possibilities, self.duration
        );
        if self.solutions.is_empty() {
            None
        } else {
            println!("Found {} solutions", self.solutions.len());
            self.solutions.sort_by_key(|solution| solution.costs());
            self.solutions.into_iter().next()
        }
    }

    fn solve_iterative(&mut self, map: &Map, ttl: usize) {
        let mut states: Vec<(Vec<Move>, Map)> = vec![(vec![], map.clone())];
        for _ in 0..=ttl {
            states = self.round(states);
        }
        self.rest_possibilities = states.len();
    }

    fn round(&mut self, states: Vec<(Vec<Move>, Map)>) -> Vec<(Vec<Move>, Map)> {
        let mut next_states: Vec<(Vec<Move>, Map)> = vec![];
        for state in states {
            self.do_step(state.0, state.1, &mut next_states);
        }
        next_states
    }

    fn do_step(&mut self, moves: Vec<Move>, map: Map, next_states: &mut Vec<(Vec<Move>, Map)>) {
        let debug = if !self.moves_search.is_empty() && self.moves_search == moves {
            println!("State!\n{}", map);
            true
        } else {
            false
        };

        if map.is_solved() {
            if debug {
                println!("Solved!");
            }
            self.solutions.push(Solution::new(moves));
            return;
        }

        let current_costs = Costs::new(&self.moves);
        if let Some(costs) = self.tried.get(map.solve_state()) {
            if current_costs >= costs {
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

        let possible_moves = map.possible_moves();
        next_states.reserve(possible_moves.len());
        for m in possible_moves.iter() {
            let mut map = map.clone();
            map.apply_move(*m);

            if debug {
                println!();
                println!("Possible Move:");
                println!("{}", map);
            } else {
                self.steps += 1;
                let mut next_moves = moves.clone();
                next_moves.push(*m);
                next_states.push((next_moves, map));
            }
        }
    }

    fn solve_recursive(&mut self, map: &Map, ttl: usize) {
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
                self.solve_recursive(&map, ttl - 1);
                self.moves.pop();
            }
        }
    }
}
