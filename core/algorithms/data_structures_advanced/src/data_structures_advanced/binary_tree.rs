// BinaryTree: ABB sin capacidad contractual. Raíz embebida en el árbol y
// contador de valores distintos; los duplicados no crecen el tamaño.

use super::tree_node::TreeNode;

/// Árbol binario de búsqueda construido desde cero.
#[allow(dead_code)] // Esqueleto del paso 4b: la raíz se usa en el paso 5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryTree {
    root: Option<Box<TreeNode>>,
    count: usize,
}

impl BinaryTree {
    /// Crea un árbol vacío utilizable (el `init()` de la especificación).
    pub fn new() -> Self {
        Self {
            root: None,
            count: 0,
        }
    }

    /// Inserta `value`; un duplicado también es éxito y no crece el tamaño.
    ///
    /// Esqueleto del paso 4b: devuelve el fallo sin resolver el caso.
    pub fn insert(&mut self, value: i32) -> bool {
        let _ = value;
        false
    }

    /// Éxito si `value` pertenece al árbol; sigue la invariante del ABB.
    ///
    /// Esqueleto del paso 4b: devuelve el fallo sin resolver el caso.
    pub fn contains(&self, value: i32) -> bool {
        let _ = value;
        false
    }

    /// `true` solo si no hay nodos.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Número de valores distintos del árbol.
    pub fn size(&self) -> usize {
        self.count
    }
}
