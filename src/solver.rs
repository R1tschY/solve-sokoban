use crate::algos::dijkstra::shortest_path;
use crate::{Costs, Map, Move, Pos, Solution, SolveState};
use likely_stable::unlikely;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct StepState {
    moves: Vec<Move>,
    map: Map,
    costs: Costs,
}

impl StepState {
    fn start(map: Map) -> Self {
        Self {
            moves: vec![],
            map,
            costs: Costs::zero(),
        }
    }
}

pub struct Solver {
    duration: Option<Duration>,
    rest_possibilities: usize,
    steps: usize,
    tried: HashMap<SolveState, Costs>,
    solutions: Vec<Solution>,

    pub moves_search: Vec<Move>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            duration: None,
            steps: 0,
            rest_possibilities: 0,
            tried: HashMap::new(),
            solutions: Vec::new(),
            moves_search: Vec::new(),
        }
    }

    pub fn solve(mut self, map: &Map, ttl: usize) -> Option<Solution> {
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
        let start = Instant::now();
        let mut states: Vec<StepState> = vec![StepState::start(map.clone())];
        for i in 0..=ttl {
            states = self.round(states);
            println!(
                "{}: {} {} ({:?})",
                i,
                ".".repeat((states.len() as f32).log10() as usize),
                states.len(),
                Instant::now().duration_since(start)
            )
        }
        self.rest_possibilities = states.len();
    }

    fn round(&mut self, states: Vec<StepState>) -> Vec<StepState> {
        let mut next_states: Vec<StepState> = vec![];
        for state in states {
            self.do_step(state, &mut next_states);
        }
        next_states
    }

    fn do_step(&mut self, current_state: StepState, next_states: &mut Vec<StepState>) {
        let debug = if !self.moves_search.is_empty() && self.moves_search == current_state.moves {
            println!("State!\n{}", current_state.map);
            true
        } else {
            false
        };

        if unlikely(current_state.map.is_solved()) {
            if debug {
                println!("Solved!");
            }
            self.solutions
                .push(Solution::new(current_state.moves, current_state.costs));
            return;
        }

        if let Some(costs) = self.tried.get(current_state.map.solve_state()) {
            if current_state.costs >= *costs {
                if debug {
                    println!("I was already here!");
                }
                return;
            }
            if debug {
                println!("I have less costs!");
            }
        }
        self.tried
            .insert(current_state.map.solve_state().clone(), current_state.costs);

        let possible_moves = current_state.map.possible_moves();
        next_states.reserve(possible_moves.len());
        for m in possible_moves.iter() {
            if debug {
                let mut map = current_state.map.clone();
                map.apply_move(*m);
                println!();
                println!("Possible Move:");
                println!("{}", map);
            }

            if !current_state.map.is_destination(m.end)
                && !current_state.map.is_box_movable_at(m.end)
            {
                if debug {
                    println!("Is a deadlock");
                }
                continue;
            }

            let next_player_pos = Pos {
                x: 2 * m.start.x - m.end.x,
                y: 2 * m.start.y - m.end.y,
            };

            let moves_cost = if let Some(cost) = shortest_path(
                &current_state.map,
                current_state.map.player(),
                next_player_pos,
            ) {
                if debug {
                    println!("Moves: {}", cost);
                }
                cost
            } else {
                if debug {
                    println!("No way!");
                }
                continue;
            };

            let mut map = current_state.map.clone();
            map.apply_move(*m);

            if !debug {
                self.steps += 1;
                let mut next_moves = current_state.moves.clone();
                next_moves.push(*m);
                next_states.push(StepState {
                    moves: next_moves,
                    map,
                    costs: Costs {
                        pushes: current_state.costs.pushes + 1,
                        moves: current_state.costs.moves + moves_cost + 1,
                    },
                });
            }
        }
    }

    // fn solve_recursive(&mut self, map: &Map, ttl: usize) {
    //     let debug = if !self.moves_search.is_empty() && self.moves_search == self.moves {
    //         println!("State!\n{}", map);
    //         true
    //     } else {
    //         false
    //     };
    //
    //     if map.is_solved() {
    //         if debug {
    //             println!("Solved!");
    //         }
    //         self.solutions.push(Solution::new(self.moves.clone()));
    //         return;
    //     }
    //
    //     if ttl == 0 && !debug {
    //         self.rest_possibilities += 1;
    //         return;
    //     }
    //
    //     let current_costs = Costs::new(&self.moves);
    //     if let Some(costs) = self.tried.get(map.solve_state()) {
    //         if current_costs >= *costs {
    //             if debug {
    //                 println!("I was already here!");
    //             }
    //             return;
    //         }
    //         if debug {
    //             println!("I have less costs!");
    //         }
    //     }
    //     self.tried.insert(map.solve_state().clone(), current_costs);
    //
    //     let moves = map.possible_moves();
    //     for m in moves.iter() {
    //         let mut map = map.clone();
    //         map.apply_move(*m);
    //
    //         if debug {
    //             println!();
    //             println!("Possible Move:");
    //             println!("{}", map);
    //         } else {
    //             self.steps += 1;
    //             self.moves.push(*m);
    //             self.solve_recursive(&map, ttl - 1);
    //             self.moves.pop();
    //         }
    //     }
    // }
}
