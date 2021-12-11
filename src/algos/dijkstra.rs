use crate::algos::matrix::Matrix;
use crate::{Map, Pos};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::ops::{Index, IndexMut};

pub type Cost = u16;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    cost: Cost,
    position: Pos,
}

struct DistMap(Matrix<Cost>);

impl DistMap {
    pub fn new(map: &Map) -> Self {
        Self(Matrix::fill(u16::MAX, map.width(), map.height()))
    }
}

impl IndexMut<Pos> for DistMap {
    fn index_mut(&mut self, index: Pos) -> &mut Cost {
        &mut self.0[(index.x as usize, index.y as usize)]
    }
}

impl Index<Pos> for DistMap {
    type Output = Cost;

    fn index(&self, index: Pos) -> &Cost {
        &self.0[(index.x as usize, index.y as usize)]
    }
}

pub struct PathGraph {
    edges: HashMap<Pos, Vec<Pos>>,
}

impl PathGraph {
    pub fn new(map: &Map) -> Self {
        let mut edges: HashMap<Pos, Vec<Pos>> =
            HashMap::with_capacity((map.height() * map.width()) / 2);
        for y in 0..map.height() {
            for x in 0..map.width() {
                let pos = Pos::new(x as u8, y as u8);
                if !map.is_wall(pos) {
                    edges.insert(pos, Self::calc_edges(map, pos));
                }
            }
        }

        Self { edges }
    }

    fn calc_edges(map: &Map, pos: Pos) -> Vec<Pos> {
        let mut res = Vec::with_capacity(4);
        if map.is_free(pos.up()) {
            res.push(pos.up());
        }
        if map.is_free(pos.down()) {
            res.push(pos.down());
        }
        if map.is_free(pos.left()) {
            res.push(pos.left());
        }
        if map.is_free(pos.right()) {
            res.push(pos.right());
        }
        res
    }

    pub fn edges(&self, pos: Pos) -> &[Pos] {
        &self.edges[&pos]
    }
}

/// Dijkstra's shortest path algorithm.
pub fn shortest_path(map: &Map, graph: &PathGraph, start: Pos, goal: Pos) -> Option<u16> {
    let mut dist: DistMap = DistMap::new(map);
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse(State {
        cost: 0,
        position: start,
    }));

    while let Some(Reverse(State { cost, position })) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in graph.edges(position) {
            let next = State {
                cost: cost + 1,
                position: edge.clone(),
            };

            if next.cost < dist[next.position] {
                heap.push(Reverse(next));
                dist[next.position] = next.cost;
            }
        }
    }

    None
}
