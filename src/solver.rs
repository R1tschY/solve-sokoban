use std::cmp::Ordering;
use crate::algos::dijkstra::{shortest_path, PathGraph};
use crate::{Costs, Map, Move, Pos, Solution, SolveState};
use likely_stable::unlikely;
use std::collections::{BinaryHeap, HashMap};
use std::time::{Duration, Instant};

struct StepState {
    moves: Vec<Move>,
    map: Map,
    costs: Costs,
}

impl Eq for StepState {}

impl Ord for StepState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.costs.cmp(&self.costs)
    }
}

impl PartialEq<StepState> for StepState {
    fn eq(&self, other: &Self) -> bool {
        self.costs == other.costs
    }
}

impl PartialOrd for StepState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.costs.partial_cmp(&self.costs)
    }
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

    pub moves_search: Vec<Move>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            duration: None,
            steps: 0,
            rest_possibilities: 0,
            tried: HashMap::new(),
            moves_search: Vec::new(),
        }
    }

    pub fn solve(mut self, map: &Map) -> Option<Solution> {
        let start = Instant::now();
        let solution = self.solve_iterative(map);
        self.duration = Some(Instant::now().duration_since(start));
        println!(
            " ==> Stats: Steps={} RestPossibilities={} Duration={:?}",
            self.steps, self.rest_possibilities, self.duration
        );
        solution
    }

    fn solve_iterative(&mut self, map: &Map) -> Option<Solution> {
        let mut queue = BinaryHeap::<StepState>::new();
        queue.push(StepState::start(map.clone()));

        while let Some(state) = queue.pop() {
            if let Some(solution) = self.do_step(state, &mut queue) {
                return Some(solution);
            }
        }

        None
    }

    fn do_step(&mut self, current_state: StepState, next_states: &mut BinaryHeap<StepState>) -> Option<Solution> {
        if unlikely(current_state.map.is_solved()) {
            return Some(Solution::new(current_state.moves, current_state.costs));
        }

        if let Some(costs) = self.tried.get(current_state.map.solve_state()) {
            if current_state.costs >= *costs {
                return None;
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

        None
    }
}
