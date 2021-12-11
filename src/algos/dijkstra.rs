use crate::{Map, Pos};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::ops::{Add, Index, IndexMut};

pub type Cost = u16;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    cost: Cost,
    position: Pos,
}

struct DistMap {
    dist: Box<[Cost]>,
    map_width: usize,
}

impl DistMap {
    pub fn new(map: &Map) -> Self {
        Self {
            dist: (0..map.size()).map(|_| u16::MAX).collect(),
            map_width: map.width(),
        }
    }
}

impl IndexMut<Pos> for DistMap {
    fn index_mut(&mut self, index: Pos) -> &mut Cost {
        &mut self.dist[index.x as usize + self.map_width * index.y as usize]
    }
}

impl Index<Pos> for DistMap {
    type Output = Cost;

    fn index(&self, index: Pos) -> &Cost {
        &self.dist[index.x as usize + self.map_width * index.y as usize]
    }
}

fn edges(map: &Map, pos: Pos) -> Vec<Pos> {
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

/// Dijkstra's shortest path algorithm.
pub fn shortest_path(map: &Map, start: Pos, goal: Pos) -> Option<u16> {
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

        for edge in edges(map, position) {
            let next = State {
                cost: cost + 1,
                position: edge,
            };

            if next.cost < dist[next.position] {
                heap.push(Reverse(next));
                dist[next.position] = next.cost;
            }
        }
    }

    None
}
