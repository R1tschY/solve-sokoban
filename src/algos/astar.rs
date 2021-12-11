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
