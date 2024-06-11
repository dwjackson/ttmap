/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

#[derive(Debug, Clone, Copy, PartialEq)]
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
        let node = &self.nodes[handle1.0];
        node.has_edge(handle2)
    }

    pub fn data(&self, handle: NodeHandle) -> &T {
        &self.nodes[handle.0].data
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
}
