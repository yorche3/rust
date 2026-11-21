// Graph: grafo dirigido sin aristas paralelas y con bucles permitidos.
//
// Registro enlazado canónico de vértices escrito a mano (sin `HashMap` ni
// índices por etiqueta) y contadores separados de vértices y aristas.

use super::graph_node::GraphNode;

/// Grafo dirigido construido desde cero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    vertices: Option<Box<GraphNode>>,
    vertex_total: usize,
    edge_total: usize,
}

impl Graph {
    /// Crea un grafo vacío utilizable (el `init()` de la especificación).
    pub fn new() -> Self {
        Self {
            vertices: None,
            vertex_total: 0,
            edge_total: 0,
        }
    }

    fn find_vertex(&self, label: i32) -> Option<&GraphNode> {
        let mut current = self.vertices.as_deref();
        while let Some(node) = current {
            if node.value() == label {
                return Some(node);
            }
            current = node.next_vertex();
        }
        None
    }

    fn find_vertex_mut(&mut self, label: i32) -> Option<&mut GraphNode> {
        let mut current = self.vertices.as_deref_mut();
        while let Some(node) = current {
            if node.value() == label {
                return Some(node);
            }
            current = node.next_vertex_mut();
        }
        None
    }

    /// Registra `label`; un duplicado también es éxito y conserva los vecinos.
    pub fn add_vertex(&mut self, label: i32) -> bool {
        if self.find_vertex(label).is_some() {
            return true;
        }
        let mut new_vertex = Box::new(GraphNode::new(label));
        new_vertex.set_next_vertex(self.vertices.take());
        self.vertices = Some(new_vertex);
        self.vertex_total += 1;
        true
    }

    /// Añade la arista `u → v`; falla si falta un extremo y no muta.
    pub fn add_edge(&mut self, u: i32, v: i32) -> bool {
        if self.find_vertex(u).is_none() || self.find_vertex(v).is_none() {
            return false;
        }
        let added = match self.find_vertex_mut(u) {
            Some(u_vertex) => u_vertex.add_neighbor(v),
            None => return false,
        };
        if added {
            self.edge_total += 1;
        }
        true
    }

    /// Éxito si la arista `u → v` existe; falla si falta un extremo.
    pub fn has_edge(&self, u: i32, v: i32) -> bool {
        match self.find_vertex(u) {
            Some(u_vertex) => u_vertex.neighbors().contains(&v),
            None => false,
        }
    }

    /// Vecinos de `u` en orden creciente como copia independiente.
    ///
    /// `Some(vec![])` es el vértice aislado; `None`, el vértice inexistente.
    pub fn neighbors(&self, u: i32) -> Option<Vec<i32>> {
        match self.find_vertex(u) {
            Some(u_vertex) => Some(u_vertex.neighbors().to_vec()),
            None => None,
        }
    }

    /// `true` solo si no hay vértices.
    pub fn is_empty(&self) -> bool {
        self.vertex_total == 0
    }

    /// Número de vértices registrados.
    pub fn vertex_count(&self) -> usize {
        self.vertex_total
    }

    /// Número de aristas registradas.
    pub fn edge_count(&self) -> usize {
        self.edge_total
    }
}
