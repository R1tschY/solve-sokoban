use solve_sokoban::algos::dijkstra::{shortest_path, PathGraph};
use solve_sokoban::solver::Solver;
use solve_sokoban::{Costs, Input, Map, Pos};
use std::str::FromStr;

fn main() {
    let input = Input::from_str(include_str!("../tests/ttac2021/level2.txt"));
    // let input = Input::from_str(include_str!("../tests/xsokoban/screen.1"));
    let map = Map::from(input.unwrap());

    println!("Map:\n{}", map);
    if let Some(solution) = Solver::new().solve(&map) {
        println!("{:?}", solution.costs());

        let mut costs = Costs::zero();
        let mut map = map.clone();
        for (i, mv) in solution.moves().iter().enumerate() {
            let push_player_pos = Pos {
                x: 2 * mv.start.x - mv.end.x,
                y: 2 * mv.start.y - mv.end.y,
            };

            costs.moves +=
                shortest_path(&map, &PathGraph::new(&map), map.player(), push_player_pos).unwrap();

            costs.pushes += 1;
            costs.moves += 1;

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
