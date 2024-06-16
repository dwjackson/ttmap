/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeHandle(usize);

#[derive(Debug)]
struct Node<T> {
    data: T,
    edges: Vec<NodeHandle>,
}

impl<T> Node<T> {
    fn add_edge(&mut self, node_handle: NodeHandle) {
        self.edges.push(node_handle);
    }

    fn remove_edge(&mut self, node_handle: NodeHandle) {
        if let Some(index) = self.edges.iter().position(|&x| x == node_handle) {
            self.edges.swap_remove(index);
        }
    }

    fn has_edge(&self, handle: NodeHandle) -> bool {
        self.edges.contains(&handle)
    }
}

#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, data: T) -> NodeHandle {
        let index = self.nodes.len();
        self.nodes.push(Node {
            data,
            edges: Vec::new(),
        });
        NodeHandle(index)
    }

    pub fn add_edge(&mut self, node_handle_1: NodeHandle, node_handle_2: NodeHandle) {
        self.nodes[node_handle_1.0].add_edge(node_handle_2);
        self.nodes[node_handle_2.0].add_edge(node_handle_1);
    }

    pub fn remove_edge(&mut self, node_handle_1: NodeHandle, node_handle_2: NodeHandle) {
        self.nodes[node_handle_1.0].remove_edge(node_handle_2);
        self.nodes[node_handle_2.0].remove_edge(node_handle_1);
    }

    pub fn is_edge_between(&self, handle1: NodeHandle, handle2: NodeHandle) -> bool {
        let n1 = &self.nodes[handle1.0];
        if n1.has_edge(handle2) {
            return true;
        }
        let n2 = &self.nodes[handle2.0];
        n2.has_edge(handle1)
    }

    pub fn data(&self, handle: NodeHandle) -> &T {
        &self.nodes[handle.0].data
    }

    pub fn find_cycles(&self) -> Vec<Vec<NodeHandle>> {
        let mut cycles = Vec::new();
        let mut visited = vec![false; self.nodes.len()];
        for i in 0..self.nodes.len() {
            let h = NodeHandle(i);
            if visited[h.0] {
                // Skip visited nodes
                continue;
            }

            // Find cycles containing this node
            let mut stack = Vec::new();
            let mut seen = HashSet::new();
            self.find_cycles_rec(h, &mut seen, &mut stack, &mut cycles);

            // Mark all nodes in cycles as visited
            for cycle in cycles.iter() {
                for ch in cycle.iter() {
                    visited[ch.0] = true;
                }
            }
        }
        cycles
    }

    fn find_cycles_rec(
        &self,
        handle: NodeHandle,
        seen: &mut HashSet<NodeHandle>,
        stack: &mut Vec<NodeHandle>,
        cycles: &mut Vec<Vec<NodeHandle>>,
    ) -> bool {
        if stack.iter().any(|h| *h == handle) {
            // Cycle found
            let mut cycle = Vec::new();
            for h in stack.iter().rev() {
                cycle.push(*h);
                if *h == handle {
                    break;
                }
            }
            cycles.push(cycle);
            return true;
        }

        let is_back_ref = !stack.is_empty();
        let back_ref = if is_back_ref {
            Some(*stack.last().unwrap())
        } else {
            None
        };

        stack.push(handle);
        let mut cycle_found = false;
        for e in self.nodes[handle.0].edges.iter() {
            if is_back_ref && back_ref.unwrap() == *e || seen.contains(&handle) {
                continue;
            }
            if self.find_cycles_rec(*e, seen, stack, cycles) {
                cycle_found = true;
            }
        }
        stack.pop();
        seen.insert(handle);
        cycle_found
    }

    pub fn connected_components(&self) -> Vec<Vec<NodeHandle>> {
        let mut connected_components = Vec::new();
        let mut visited = vec![false; self.nodes.len()];
        for i in 0..self.nodes.len() {
            // Skip already-visited nodes
            if visited[i] {
                continue;
            }
            let h = NodeHandle(i);
            let component = self.bfs(h, &mut visited);
            connected_components.push(component);
        }
        connected_components
    }

    fn bfs(&self, start: NodeHandle, visited: &mut [bool]) -> Vec<NodeHandle> {
        let mut nodes = Vec::new();
        let mut stack = vec![start];
        while let Some(h) = stack.pop() {
            if visited[h.0] {
                continue;
            }
            nodes.push(h);
            visited[h.0] = true;
            for e in self.nodes[h.0].edges.iter() {
                stack.push(*e);
            }
        }
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_with_two_connected_nodes() {
        let mut g: Graph<i32> = Graph::new();
        let n1 = g.add_node(123);
        let n2 = g.add_node(456);
        g.add_edge(n1, n2);

        assert!(g.is_edge_between(n1, n2));
        assert!(g.is_edge_between(n2, n1));
    }

    #[test]
    fn test_remove_edge() {
        let mut g: Graph<i32> = Graph::new();
        let n1 = g.add_node(123);
        let n2 = g.add_node(456);
        g.add_edge(n1, n2);
        g.remove_edge(n1, n2);

        assert!(!g.is_edge_between(n1, n2));
        assert!(!g.is_edge_between(n2, n1));
    }

    #[test]
    fn test_find_simple_cycle() {
        let mut g: Graph<i32> = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);
        g.add_edge(n1, n2);
        g.add_edge(n2, n3);
        g.add_edge(n3, n4);
        g.add_edge(n4, n1);
        let cycles = g.find_cycles();
        assert_eq!(cycles.len(), 1);
    }

    /*
     * This has 3 cycles, one of which contains the others:
     * 0---1---2
     * |       |
     * 3---4---5
     *     |   |
     *     6---7
     */
    #[test]
    fn test_find_cyles() {
        let mut g: Graph<i32> = Graph::new();
        let n0 = g.add_node(0);
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);
        let n5 = g.add_node(5);
        let n6 = g.add_node(6);
        let n7 = g.add_node(7);
        g.add_edge(n0, n1);
        g.add_edge(n0, n3);
        g.add_edge(n1, n2);
        g.add_edge(n2, n5);
        g.add_edge(n5, n4);
        g.add_edge(n5, n7);
        g.add_edge(n4, n3);
        g.add_edge(n6, n4);
        g.add_edge(n7, n6);
        let cycles = g.find_cycles();
        assert_eq!(cycles.len(), 2);

        match cycles.iter().find(|c| c.len() == 6) {
            Some(c) => {
                let node_set: HashSet<NodeHandle> = c.iter().map(|x| *x).collect();
                let correct_nodes = [0, 1, 2, 3, 4, 5];
                for x in correct_nodes.into_iter() {
                    assert!(node_set.contains(&NodeHandle(x)));
                }
            }
            None => panic!("Large rectangle wasn't found"),
        }

        match cycles.iter().find(|c| c.len() == 4) {
            Some(c) => {
                let node_set: HashSet<NodeHandle> = c.iter().map(|x| *x).collect();
                let correct_nodes = [5, 4, 6, 7];
                for x in correct_nodes.into_iter() {
                    assert!(node_set.contains(&NodeHandle(x)));
                }
            }
            None => panic!("Small square wasn't found"),
        }
    }

    #[test]
    fn test_node_handle_equals() {
        let h1 = NodeHandle(2);
        let h2 = NodeHandle(2);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_connected_components() {
        let mut g: Graph<i32> = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        let n4 = g.add_node(4);
        g.add_edge(n1, n2);
        g.add_edge(n3, n4);
        let cc = g.connected_components();
        assert_eq!(cc.len(), 2);
        let c1 = &cc[0];
        let correct_nodes1 = [0, 1];
        assert_eq!(c1.len(), correct_nodes1.len());
        for x in correct_nodes1.into_iter() {
            assert!(c1.contains(&NodeHandle(x)));
        }
        let c2 = &cc[1];
        let correct_nodes2 = [2, 3];
        assert_eq!(c2.len(), correct_nodes2.len());
        for x in correct_nodes2.into_iter() {
            assert!(c2.contains(&NodeHandle(x)));
        }
    }
}
