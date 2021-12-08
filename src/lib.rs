use std::fmt;
use std::fmt::{Formatter, Write};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq)]
enum CellState {
    Empty,
    Wall,
    Player,
    Box,
    Destination,
    BoxOnDestination,
    PlayerOnDestination,
}

impl CellState {
    pub fn from_char(c: char) -> Result<Self, InputError> {
        use CellState::*;

        match c {
            ' ' => Ok(Empty),
            '#' => Ok(Wall),
            '@' => Ok(Player),
            '$' => Ok(Box),
            '.' => Ok(Destination),
            '*' => Ok(BoxOnDestination),
            '+' => Ok(PlayerOnDestination),
            _ => Err(InputError::UnknownChar(c)),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            CellState::Empty => ' ',
            CellState::Wall => '#',
            CellState::Player => '@',
            CellState::Box => '$',
            CellState::Destination => '.',
            CellState::BoxOnDestination => '*',
            CellState::PlayerOnDestination => '+',
        }
    }
}

#[derive(Debug)]
pub enum InputError {
    UnknownChar(char),
    MissingPlayer,
}

pub struct Input {
    input: Vec<Vec<CellState>>,
}

impl Input {
    pub fn get_player_pos(&self) -> Result<Pos, InputError> {
        for (y, line) in self.input.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == CellState::Player || *cell == CellState::PlayerOnDestination {
                    return Ok(Pos {
                        x: x as u8,
                        y: y as u8,
                    });
                }
            }
        }

        Err(InputError::MissingPlayer)
    }

    pub fn get_boxes(&self) -> Vec<Pos> {
        let mut boxes = vec![];
        for (y, line) in self.input.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == CellState::Box || *cell == CellState::BoxOnDestination {
                    boxes.push(Pos {
                        x: x as u8,
                        y: y as u8,
                    });
                }
            }
        }
        boxes
    }

    pub fn get_destinations(&self) -> Vec<Pos> {
        let mut boxes = vec![];
        for (y, line) in self.input.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == CellState::Destination
                    || *cell == CellState::BoxOnDestination
                    || *cell == CellState::PlayerOnDestination
                {
                    boxes.push(Pos {
                        x: x as u8,
                        y: y as u8,
                    });
                }
            }
        }
        boxes
    }
}

impl std::str::FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Result<Vec<Vec<CellState>>, InputError> = s
            .split('\n')
            .map(|line| line.chars().map(|c| CellState::from_char(c)).collect())
            .collect();
        Ok(Self {
            input: input.unwrap(),
        })
    }
}

#[derive(Clone)]
struct MapProps {
    width: usize,
    height: usize,
    map: Vec<bool>,
    destinations: Vec<Pos>,
}

#[derive(Clone)]
pub struct Map {
    props: Rc<MapProps>,
    solve_state: SolveState,
}

impl Map {
    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(self.solve_state.boxes.len() * 4);
        for b in self.solve_state.boxes.iter().copied() {
            if self.is_free(b.up()) && self.is_free(b.down()) {
                moves.push(Move::new(b, b.down()));
                moves.push(Move::new(b, b.up()));
            }
            if self.is_free(b.left()) && self.is_free(b.right()) {
                moves.push(Move::new(b, b.right()));
                moves.push(Move::new(b, b.left()));
            }
        }
        moves
    }

    pub fn is_free(&self, pos: Pos) -> bool {
        !self.is_wall(pos) && !self.solve_state.boxes.contains(&pos)
    }

    pub fn is_wall(&self, pos: Pos) -> bool {
        self.props
            .map
            .get(pos.x as usize + pos.y as usize * self.props.width)
            .copied()
            .unwrap_or_default()
    }

    pub fn apply_move(&mut self, m: Move) {
        self.solve_state.apply_move(m);
    }

    pub fn is_solved(&self) -> bool {
        self.solve_state.boxes == self.props.destinations
    }

    pub fn solve_state(&self) -> &SolveState {
        &self.solve_state
    }

    pub fn destinations(&self) -> &[Pos] {
        &self.props.destinations
    }

    pub fn boxes(&self) -> &[Pos] {
        &self.solve_state.boxes
    }

    pub fn player(&self) -> Pos {
        self.solve_state.player
    }
}

impl From<Input> for Map {
    fn from(input: Input) -> Self {
        let width = input.input[0].len();
        let height = input.input.len();

        let mut map = vec![false; width * height];
        for (x, line) in input.input.iter().enumerate() {
            for (y, cell) in line.iter().enumerate() {
                map[x + width * y] = *cell == CellState::Wall;
            }
        }

        let mut boxes = input.get_boxes();
        boxes.sort();

        let mut destinations = input.get_destinations();
        destinations.sort();

        Map {
            props: Rc::new(MapProps {
                width,
                height,
                map: map.into_boxed_slice().into(),
                destinations,
            }),
            solve_state: SolveState {
                player: input.get_player_pos().unwrap(),
                boxes,
            },
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.props.height {
            for x in 0..self.props.width {
                let pos = Pos {
                    x: x as u8,
                    y: y as u8,
                };
                if self.props.map[x + self.props.width * y] {
                    f.write_char('#')?;
                } else if self.solve_state.player == pos {
                    f.write_char('@')?;
                } else if self.solve_state.boxes.contains(&pos) {
                    f.write_char('o')?;
                } else if self.props.destinations.contains(&pos) {
                    f.write_char('!')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

impl Pos {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Move {
    pub start: Pos,
    pub end: Pos,
}

impl Move {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self { start, end }
    }
}

#[derive(Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
pub struct Costs {
    pub pushes: u32,
    pub moves: u32,
}

impl Costs {
    pub fn new(moves: &[Move]) -> Self {
        Self {
            pushes: moves.len() as u32,
            moves: 0,
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    moves: Vec<Move>,
    costs: Costs,
}

impl Solution {
    pub fn new(moves: Vec<Move>) -> Self {
        let costs = Costs::new(&moves);
        Self { moves, costs }
    }

    pub fn costs(&self) -> Costs {
        self.costs
    }

    pub fn moves(&self) -> &[Move] {
        &self.moves
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct SolveState {
    player: Pos,
    boxes: Vec<Pos>,
}

impl SolveState {
    pub fn apply_move(&mut self, m: Move) {
        let b = self
            .boxes
            .iter_mut()
            .find(|b| b == &&m.start)
            .expect("impossible move: box does not exist");
        *b = m.end;

        self.player = m.start;
        self.boxes.sort();
    }
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// struct AStarEdge<I: Copy + Eq> {
//     cost: u32,
//     node_id: I,
// }
//
// pub trait AStarMap {
//     type Index: Copy + Eq;
//
//     fn neighbors(&self, node: Self::Index) -> Vec<Self::Index>;
//     fn edge_weight(&self, from: Self::Index, to: Self::Index) -> u32;
//     fn heuristic(&self, node: Self::Index, goal: Self::Index);
// }
//
// struct AStar<T, I> {
//     open_list: BinaryHeap<AStarEdge<I>>,
//     g_score: HashMap<I, u32>,
//     f_score: HashMap<I, u32>,
//     came_from: HashMap<I, I>,
// }
//
// impl<M: AStarMap<Index = I>, I: Eq + Hash + Copy> AStar<M, I> {
//     fn new() -> Self {
//         Self {
//             open_list: BinaryHeap::new(),
//             g_score: Default::default(),
//             f_score: Default::default(),
//             came_from: Default::default(),
//         }
//     }
//
//     fn solve(mut self, map: &M, start: I, goal: I) -> Option<u32> {
//         self.open_list.push(AStarEdge {
//             cost: 0,
//             node_id: start,
//         });
//
//         while let Some(AStarEdge { cost, node_id }) = self.open_list.pop() {
//             if node_id == goal {
//                 return Some(cost);
//             }
//
//             self.open_list.remove(node_id);
//             for neighbor in map.neighbors(node_id) {
//                 let tentative_g_score = self.g_score[node_id] + map.edge_weight(node_id, neighbor);
//                 if tentative_g_score < self.g_score[neighbor] {
//                     self.came_from.insert(neighbor, node_id);
//                     self.g_score.insert(neighbor, tentative_g_score);
//                     self.f_score
//                         .insert(neighbor, tentative_g_score + map.heuristic(neighbor, goal));
//                     if self.open_list.contains()
//                 }
//             }
//         }
//
//         None
//     }
// }
//
// fn shortest_path<M: AStarMap, I: Eq + Hash + Clone + PartialEq>(
//     map: &M,
//     start: I,
//     goal: I,
// ) -> Option<u32> {
//     AStar::new().solve(map, start, goal)
// }
