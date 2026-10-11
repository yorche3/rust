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

    /// `true` solo si no hay nodos.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Número de nodos.
    pub fn size(&self) -> usize {
        self.count
    }

    /// Apila `value`. O(1).
    pub fn push(&mut self, value: i32) {
        let mut new_node = Box::new(Node::new(value));
        new_node.set_next(self.top.take());
        self.top = Some(new_node);
        self.count += 1;
    }

    /// Desapila; -1 cuando la pila está vacía. O(1).
    pub fn pop(&mut self) -> i32 {
        if let Some(mut top_node) = self.top.take() {
            self.top = top_node.next.take();
            self.count -= 1;
            top_node.value()
        } else {
            -1
        }
    }

    /// Mira la cima sin desapilarla; -1 cuando la pila está vacía.
    pub fn peek(&self) -> i32 {
        self.top.as_deref().map_or(-1, |node| node.value())
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
