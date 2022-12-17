use crate::algos::matrix::Matrix;
use std::fmt;
use std::fmt::{Formatter, Write};
use std::hash::Hash;
use std::rc::Rc;

pub mod algos;

pub mod solver;

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
            ' ' | '-' | '_' => Ok(Empty),
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

    pub fn is_wall(self) -> bool {
        self == CellState::Wall
    }

    pub fn is_destination(self) -> bool {
        match self {
            CellState::Destination => true,
            CellState::BoxOnDestination => true,
            CellState::PlayerOnDestination => true,
            _ => false,
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

struct MapProps {
    width: usize,
    height: usize,
    map: Matrix<bool>,
    dead: Matrix<bool>,
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
            if self.is_free(b.up()) && self.is_push_target(b.down()) {
                moves.push(Move::new(b, b.down()));
            }
            if self.is_push_target(b.up()) && self.is_free(b.down()) {
                moves.push(Move::new(b, b.up()));
            }
            if self.is_push_target(b.left()) && self.is_free(b.right()) {
                moves.push(Move::new(b, b.left()));
            }
            if self.is_free(b.left()) && self.is_push_target(b.right()) {
                moves.push(Move::new(b, b.right()));
            }
        }
        moves
    }

    pub fn is_free(&self, pos: Pos) -> bool {
        !self.is_wall(pos) && !self.solve_state.boxes.contains(&pos)
    }

    pub fn is_push_target(&self, pos: Pos) -> bool {
        !self.is_dead(pos) && !self.solve_state.boxes.contains(&pos)
    }

    pub fn is_wall(&self, pos: Pos) -> bool {
        self.props
            .map
            .get(pos.x as usize, pos.y as usize)
            .copied()
            .unwrap_or_default()
    }

    pub fn is_dead(&self, pos: Pos) -> bool {
        self.props
            .dead
            .get(pos.x as usize, pos.y as usize)
            .copied()
            .unwrap_or_default()
    }

    pub fn is_destination(&self, pos: Pos) -> bool {
        self.props.destinations.contains(&pos)
    }

    pub fn apply_move(&mut self, m: Move) {
        self.solve_state.apply_move(m);
        self.solve_state.player = m.start;
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

    pub fn height(&self) -> usize {
        self.props.height
    }

    pub fn width(&self) -> usize {
        self.props.width
    }

    pub fn size(&self) -> usize {
        self.props.width * self.props.height
    }

    pub fn set_player_pos(&mut self, pos: Pos) {
        // TODO: remove me
        self.solve_state.player = pos;
    }

    fn detect_dead_positions(input: &Input, width: usize, height: usize) -> Matrix<bool> {
        let mut dead = Matrix::fill(false, width, height);
        for (y, line) in input.input.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if cell.is_wall() || x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    dead[(x, y)] = true;
                    continue
                }

                // dead corner
                if !cell.is_destination() {
                    let rblocked = input.input[y][x + 1].is_wall();
                    let lblocked = input.input[y][x - 1].is_wall();
                    let tblocked = input.input[y - 1][x].is_wall();
                    let bblocked = input.input[y + 1][x].is_wall();
                    if (rblocked || lblocked) && (tblocked || bblocked) {
                        dead[(x, y)] = true;
                        continue
                    }
                }

                // big dead corner
                if !cell.is_destination() {
                    let left = Self::vertical_dead_end(input, y, x, -1);
                    let right = Self::vertical_dead_end(input, y, x, 1);
                    let top = Self::horizontal_dead_end(input, y, x, -1);
                    let bottom = Self::horizontal_dead_end(input, y, x, 1);
                    if left || right || top || bottom {
                        dead[(x, y)] = true;
                        continue
                    }
                }
            }
        }
        dead
    }

    fn vertical_dead_end(input: &Input, y: usize, x: usize, x_offset: isize) -> bool {
        let ox = if let Some(ox) = x.checked_add_signed(x_offset) {
            ox
        } else {
            return true;
        };
        assert_ne!(y, 0);
        assert_ne!(x, 0);

        let mut i = y - 1;
        let mut top = false;
        while i >= 0 && input.input[i][ox].is_wall() {
            if input.input[i][x].is_destination() {
                break
            }
            if input.input[i][x].is_wall() {
                top = true;
                break
            }
            i -= 1;
        }

        let mut i = y + 1;
        let mut bottom = false;
        while i < input.input.len() && input.input[i][ox].is_wall() {
            if input.input[i][x].is_destination() {
                break
            }
            if input.input[i][x].is_wall() {
                bottom = true;
                break
            }
            i += 1;
        }

        top && bottom
    }

    fn horizontal_dead_end(input: &Input, y: usize, x: usize, y_offset: isize) -> bool {
        let oy = if let Some(oy) = y.checked_add_signed(y_offset) {
            oy
        } else {
            return true;
        };

        let mut i = x - 1;
        let mut left = false;
        while i >= 0 && input.input[oy][i].is_wall() {
            if input.input[y][i].is_destination() {
                break
            }
            if input.input[y][i].is_wall() {
                left = true;
                break
            }
            i -= 1;
        }

        let mut i = x + 1;
        let mut right = false;
        while i < input.input[0].len() && input.input[oy][i].is_wall() {
            if input.input[y][i].is_destination() {
                break
            }
            if input.input[y][i].is_wall() {
                right = true;
                break
            }
            i += 1;
        }

        left && right
    }
}

impl From<Input> for Map {
    fn from(input: Input) -> Self {
        let width = input
            .input
            .iter()
            .map(|x| x.len())
            .max()
            .unwrap_or_default();
        let height = input.input.len();

        let mut map = Matrix::fill(false, width, height);
        for (y, line) in input.input.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                map[(x, y)] = cell.is_wall();
            }
        }

        let dead = Self::detect_dead_positions(&input, width, height);

        let mut boxes = input.get_boxes();
        boxes.sort();

        let mut destinations = input.get_destinations();
        destinations.sort();

        Map {
            props: Rc::new(MapProps {
                width,
                height,
                map,
                dead,
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
                if self.props.map[(x, y)] {
                    f.write_char('#')?;
                } else if self.props.dead[(x, y)] {
                    f.write_char('~')?;
                } else if self.solve_state.player == pos {
                    f.write_char('@')?;
                } else if self.solve_state.boxes.contains(&pos) {
                    f.write_char('o')?;
                } else if self.props.destinations.contains(&pos) {
                    f.write_char('.')?;
                } else {
                    f.write_char(' ')?;
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

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}x{}", self.x, self.y))
    }
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
    pub pushes: u16,
    pub moves: u16,
}

impl Costs {
    pub fn new(pushes: u16, moves: u16) -> Self {
        Self { pushes, moves }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Debug)]
pub struct Solution {
    moves: Vec<Move>,
    costs: Costs,
}

impl Solution {
    pub fn new(moves: Vec<Move>, costs: Costs) -> Self {
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
