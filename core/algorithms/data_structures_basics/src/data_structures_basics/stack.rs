// Stack: pila LIFO construida desde cero sobre Node.

use super::node::Node;

/// Pila LIFO construida desde cero sobre `Node`.
pub struct Stack {
    top: Option<Box<Node>>,
    count: usize,
}

impl Stack {
    /// Crea una pila vacía (el `init()` de la especificación): sin enlaces y
    /// con el contador a cero.
    pub fn new() -> Self {
        Self {
            top: None,
            count: 0,
        }
    }

    /// Esqueleto: true solo si no hay nodos.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Esqueleto: número de nodos.
    pub fn size(&self) -> usize {
        self.count
    }

    /// Esqueleto: apila `value`. O(1).
    pub fn push(&mut self, _value: i32) {}

    /// Esqueleto: desapila; -1 cuando la pila está vacía. O(1).
    pub fn pop(&mut self) -> i32 {
        -1
    }

    /// Esqueleto: mira la cima sin desapilarla; -1 cuando la pila está vacía.
    pub fn peek(&self) -> i32 {
        -1
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
