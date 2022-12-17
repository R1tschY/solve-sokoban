use crate::algos::dijkstra::{shortest_path, PathGraph};
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
            states = self.step(states);
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

    fn step(&mut self, states: Vec<StepState>) -> Vec<StepState> {
        let mut next_states: Vec<StepState> = vec![];
        for state in states {
            self.do_step(state, &mut next_states);
        }
        next_states
    }

    fn do_step(&mut self, current_state: StepState, next_states: &mut Vec<StepState>) {
        if unlikely(current_state.map.is_solved()) {
            self.solutions
                .push(Solution::new(current_state.moves, current_state.costs));
            return;
        }

        if let Some(costs) = self.tried.get(current_state.map.solve_state()) {
            if current_state.costs >= *costs {
                return;
            }
        }
        self.tried
            .insert(current_state.map.solve_state().clone(), current_state.costs);

        let path_graph = PathGraph::new(&current_state.map);

        let possible_moves = current_state.map.possible_moves();
        next_states.reserve(possible_moves.len());
        for m in possible_moves.iter() {
            let next_player_pos = Pos {
                x: 2 * m.start.x - m.end.x,
                y: 2 * m.start.y - m.end.y,
            };

            let moves_cost = if let Some(cost) = shortest_path(
                &current_state.map,
                &path_graph,
                current_state.map.player(),
                next_player_pos,
            ) {
                cost
            } else {
                continue;
            };

            let mut map = current_state.map.clone();
            map.apply_move(*m);

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
