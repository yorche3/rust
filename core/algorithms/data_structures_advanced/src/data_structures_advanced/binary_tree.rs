// BinaryTree: ABB sin capacidad contractual. Raíz embebida en el árbol y
// contador de valores distintos; los duplicados no crecen el tamaño.

use super::tree_node::TreeNode;

/// Árbol binario de búsqueda construido desde cero.
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
    pub fn insert(&mut self, value: i32) -> bool {
        match self.root.as_deref_mut() {
            Some(node) => {
                if Self::insert_at(node, value) {
                    self.count += 1;
                }
            }
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
                self.count += 1;
            }
        }
        true
    }

    /// Desciende por la invariante del ABB y enlaza el hueco libre.
    ///
    /// Devuelve `true` solo si el valor era nuevo: un duplicado no crece el
    /// tamaño, pero tampoco es un fallo.
    fn insert_at(node: &mut TreeNode, value: i32) -> bool {
        let current = node.value();
        if value == current {
            return false;
        }
        if value < current {
            match node.left_mut() {
                Some(left) => Self::insert_at(left, value),
                None => {
                    node.set_left(Some(Box::new(TreeNode::new(value))));
                    true
                }
            }
        } else {
            match node.right_mut() {
                Some(right) => Self::insert_at(right, value),
                None => {
                    node.set_right(Some(Box::new(TreeNode::new(value))));
                    true
                }
            }
        }
    }

    /// Éxito si `value` pertenece al árbol; sigue la invariante del ABB.
    pub fn contains(&self, value: i32) -> bool {
        let mut current = self.root.as_deref();
        while let Some(node) = current {
            let node_value = node.value();
            if value == node_value {
                return true;
            }
            current = if value < node_value {
                node.left()
            } else {
                node.right()
            };
        }
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
