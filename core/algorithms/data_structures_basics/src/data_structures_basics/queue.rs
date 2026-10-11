// Queue: cola FIFO construida desde cero sobre Node.

use super::node::Node;

/// Cola FIFO construida desde cero sobre `Node`.
pub struct Queue {
    front: Option<Box<Node>>,
    count: usize,
}

impl Queue {
    /// Crea una cola vacía (el `init()` de la especificación): sin enlaces y
    /// con el contador a cero.
    pub fn new() -> Self {
        Self {
            front: None,
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

    /// Esqueleto: encola `value` al final. O(n) — ver la nota del módulo.
    pub fn enqueue(&mut self, _value: i32) {}

    /// Esqueleto: desencola; -1 cuando la cola está vacía. O(1).
    pub fn dequeue(&mut self) -> i32 {
        -1
    }

    /// Esqueleto: mira el frente sin desencolarlo; -1 cuando la cola está
    /// vacía.
    pub fn peek(&self) -> i32 {
        -1
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}
