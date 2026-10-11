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

    /// `true` solo si no hay nodos.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Número de nodos.
    pub fn size(&self) -> usize {
        self.count
    }

    /// Encola `value` al final. O(n) — ver la nota del módulo.
    pub fn enqueue(&mut self, value: i32) {
        let mut current = &mut self.front;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node::new(value)));
        self.count += 1;
    }

    /// Desencola; -1 cuando la cola está vacía. O(1).
    pub fn dequeue(&mut self) -> i32 {
        if let Some(mut front_node) = self.front.take() {
            self.front = front_node.next.take();
            self.count -= 1;
            front_node.value()
        } else {
            -1
        }
    }

    /// Mira el frente sin desencolarlo; -1 cuando la cola está vacía.
    pub fn peek(&self) -> i32 {
        self.front.as_deref().map_or(-1, |node| node.value())
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}
