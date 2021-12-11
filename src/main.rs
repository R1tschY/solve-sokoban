use likely_stable::unlikely;
use solve_sokoban::algos::dijkstra::{shortest_path, Cost};
use solve_sokoban::solver::Solver;
use solve_sokoban::{Costs, Input, Map, Move, Pos, Solution, SolveState};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn main() {
    let input = Input::from_str(include_str!("levels/level2.txt"));
    //let input = Input::from_str(include_str!("../tests/xsokoban/screen.2"));
    let map = Map::from(input.unwrap());

    if false {
        let search = vec![/*Move::new(Pos::new(3, 3), Pos::new(4, 3))*/];
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

            let mut costs = Costs::zero();
            let mut map = map.clone();
            for (i, mv) in solution.moves().iter().enumerate() {
                let push_player_pos = Pos {
                    x: 2 * mv.start.x - mv.end.x,
                    y: 2 * mv.start.y - mv.end.y,
                };
                costs.pushes += 1;
                costs.moves += shortest_path(&map, map.player(), push_player_pos).unwrap() + 1;

                println!();
                println!("Step {}:", i + 1);

                map.set_player_pos(push_player_pos);
                println!("{}", map);
                map.apply_move(*mv);
                println!("{}", map);
                println!("{:?}", costs);
            }
        }
    }
}
