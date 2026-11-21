// Graph: grafo dirigido sin aristas paralelas y con bucles permitidos.
//
// Registro enlazado canónico de vértices escrito a mano (sin `HashMap` ni
// índices por etiqueta) y contadores separados de vértices y aristas.

use super::graph_node::GraphNode;

/// Grafo dirigido construido desde cero.
#[allow(dead_code)] // Esqueleto del paso 4b: el registro se usa en el paso 5.
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

    /// Registra `label`; un duplicado también es éxito y conserva los vecinos.
    ///
    /// Esqueleto del paso 4b: devuelve el fallo sin resolver el caso.
    pub fn add_vertex(&mut self, label: i32) -> bool {
        let _ = label;
        false
    }

    /// Añade la arista `u → v`; falla si falta un extremo y no muta.
    ///
    /// Esqueleto del paso 4b: devuelve el fallo sin resolver el caso.
    pub fn add_edge(&mut self, u: i32, v: i32) -> bool {
        let (_, _) = (u, v);
        false
    }

    /// Éxito si la arista `u → v` existe; falla si falta un extremo.
    ///
    /// Esqueleto del paso 4b: devuelve el fallo sin resolver el caso.
    pub fn has_edge(&self, u: i32, v: i32) -> bool {
        let (_, _) = (u, v);
        false
    }

    /// Vecinos de `u` en orden creciente como copia independiente.
    ///
    /// `Some(vec![])` es el vértice aislado; `None`, el vértice inexistente.
    ///
    /// Esqueleto del paso 4b: devuelve la ausencia sin resolver el caso.
    pub fn neighbors(&self, u: i32) -> Option<Vec<i32>> {
        let _ = u;
        None
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
