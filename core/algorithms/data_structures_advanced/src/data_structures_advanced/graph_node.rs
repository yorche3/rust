// GraphNode: vértice del grafo. Interno a `Graph`: no se exporta.
//
// Etiqueta única, secuencia de vecinos sin duplicados en orden creciente y
// enlace al siguiente vértice del registro canónico.

use super::node::Node;

/// Vértice del registro enlazado canónico de `Graph`.
#[allow(dead_code)] // Esqueleto del paso 4b: los enlaces se usan en el paso 5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GraphNode {
    value: Node,
    neighbors: Vec<i32>,
    next_vertex: Option<Box<GraphNode>>,
}

#[allow(dead_code)] // Esqueleto del paso 4b: lo usa `Graph` en el paso 5.
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
}
