// TreeNode: nodo propio del ABB. Valor por embebido y dos enlaces que valida
// este nodo; un enlace ausente representa un subárbol vacío.

use super::node::Node;

/// Nodo del árbol binario de búsqueda.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    value: Node,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    /// Crea un nodo con `value` y ambos enlaces ausentes.
    pub fn new(value: i32) -> Self {
        Self {
            value: Node::new(value),
            left: None,
            right: None,
        }
    }

    /// Valor del nodo.
    pub fn value(&self) -> i32 {
        self.value.value()
    }

    /// Enlace izquierdo, o `None` cuando está ausente.
    pub fn left(&self) -> Option<&TreeNode> {
        self.left.as_deref()
    }

    /// Enlace derecho, o `None` cuando está ausente.
    pub fn right(&self) -> Option<&TreeNode> {
        self.right.as_deref()
    }

    /// Reemplaza el enlace izquierdo con `node`, o lo deja ausente con `None`.
    ///
    /// Esqueleto del paso 4b: el enlace se asigna en el paso 5.
    pub fn set_left(&mut self, node: Option<Box<TreeNode>>) {
        let _ = node;
    }

    /// Reemplaza el enlace derecho con `node`, o lo deja ausente con `None`.
    ///
    /// Esqueleto del paso 4b: el enlace se asigna en el paso 5.
    pub fn set_right(&mut self, node: Option<Box<TreeNode>>) {
        let _ = node;
    }
}
