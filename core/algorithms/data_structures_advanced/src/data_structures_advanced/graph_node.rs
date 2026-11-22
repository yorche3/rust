// GraphNode: vértice del grafo. Interno a `Graph`: no se exporta.
//
// Etiqueta única, secuencia de vecinos sin duplicados en orden creciente y
// enlace al siguiente vértice del registro canónico.

use super::node::Node;

/// Vértice del registro enlazado canónico de `Graph`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GraphNode {
    value: Node,
    neighbors: Vec<i32>,
    next_vertex: Option<Box<GraphNode>>,
}

impl GraphNode {
    /// Crea un vértice con `label` y sin vecinos (el `init(label)` interno).
    pub(crate) fn new(label: i32) -> Self {
        Self {
            value: Node::new(label),
            neighbors: Vec::new(),
            next_vertex: None,
        }
    }

    /// Etiqueta del vértice.
    pub(crate) fn value(&self) -> i32 {
        self.value.value()
    }

    /// Vecinos en orden creciente, sin duplicados.
    pub(crate) fn neighbors(&self) -> &[i32] {
        &self.neighbors
    }

    /// `true` si `v` ya figura en la adyacencia; se recorre a mano.
    pub(crate) fn has_neighbor(&self, v: i32) -> bool {
        let mut index = 0;
        while index < self.neighbors.len() {
            if self.neighbors[index] == v {
                return true;
            }
            index += 1;
        }
        false
    }

    /// Inserta `v` en su posición creciente.
    ///
    /// Devuelve `false` si la arista ya existía: ni duplica el vecino ni
    /// cuenta una arista nueva.
    pub(crate) fn add_neighbor(&mut self, v: i32) -> bool {
        let mut position = 0;
        while position < self.neighbors.len() && self.neighbors[position] < v {
            position += 1;
        }
        if position < self.neighbors.len() && self.neighbors[position] == v {
            return false;
        }
        self.neighbors.insert(position, v);
        true
    }

    pub(crate) fn next_vertex(&self) -> Option<&GraphNode> {
        self.next_vertex.as_deref()
    }

    pub(crate) fn next_vertex_mut(&mut self) -> Option<&mut GraphNode> {
        self.next_vertex.as_deref_mut()
    }

    pub(crate) fn set_next_vertex(&mut self, next: Option<Box<GraphNode>>) {
        self.next_vertex = next;
    }
}
