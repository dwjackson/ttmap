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
    }

    pub fn remove_edge(&mut self, node_handle_1: NodeHandle, node_handle_2: NodeHandle) {
        self.nodes[node_handle_1.0].remove_edge(node_handle_2);
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

    pub fn connected_components(&self) -> Vec<Vec<NodeHandle>> {
        let mut cc = Vec::new();
        let mut visited = vec![false; self.nodes.len()];
        for i in 0..self.nodes.len() {
            if visited[i] {
                // Skip visited nodes
                continue;
            }
            let h = NodeHandle(i);
            let component = self.bfs(h);
            for c in component.iter() {
                visited[c.0] = true;
            }
            cc.push(component);
            visited[i] = true;
        }
        cc
    }

    fn bfs(&self, start: NodeHandle) -> Vec<NodeHandle> {
        let mut nodes = vec![start];
        let mut stack = vec![start];
        let mut seen = HashSet::new();
        while let Some(h) = stack.pop() {
            if seen.contains(&h) {
                // Deal with cycles
                continue;
            }
            seen.insert(h);
            let n = &self.nodes[h.0];
            for c in n.edges.iter() {
                nodes.push(*c);
                stack.push(*c);
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
    fn test_connected_components() {
        let mut g: Graph<i32> = Graph::new();
        let n1 = g.add_node(111);
        let n2 = g.add_node(222);
        let n3 = g.add_node(333);
        let n4 = g.add_node(444);
        g.add_edge(n1, n2);
        g.add_edge(n3, n4);
        let connected_components = g.connected_components();
        assert_eq!(connected_components.len(), 2);
        for cc in connected_components.iter() {
            assert_eq!(cc.len(), 2);
        }
    }
}
