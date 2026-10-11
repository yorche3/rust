// LinkedList: lista enlazada simple construida desde cero sobre Node.

use super::node::Node;

/// Lista enlazada simple construida desde cero sobre `Node`.
pub struct LinkedList {
    head: Option<Box<Node>>,
    count: usize,
}

impl LinkedList {
    /// Crea una lista vacía (el `init()` de la especificación): sin enlaces y
    /// con el contador a cero.
    pub fn new() -> Self {
        Self {
            head: None,
            count: 0,
        }
    }

    /// Valor de la cabeza, o -1 cuando la lista está vacía.
    pub fn head(&self) -> i32 {
        self.head.as_deref().map_or(-1, |node| node.value())
    }

    /// `true` solo si no hay nodos.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Número de nodos.
    pub fn size(&self) -> usize {
        self.count
    }

    /// Inserta `value` al principio. O(1).
    pub fn insert_head(&mut self, value: i32) {
        let mut new_node = Box::new(Node::new(value));
        new_node.set_next(self.head.take());
        self.head = Some(new_node);
        self.count += 1;
    }

    /// Inserta `value` al final. O(n) — ver la nota del módulo.
    pub fn insert_tail(&mut self, value: i32) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node::new(value)));
        self.count += 1;
    }

    /// Borra la primera aparición de `value`; `true` si borró y `false` si el
    /// valor no estaba. O(n).
    pub fn delete(&mut self, value: i32) -> bool {
        let mut link = &mut self.head;
        loop {
            // La decisión se toma con un préstamo inmutable que termina antes
            // de reenganchar; así el préstamo mutable del enlace no se solapa.
            let matches = link.as_ref().is_some_and(|node| node.value() == value);
            if matches {
                let removed = link.take();
                *link = removed.and_then(|mut node| node.next.take());
                self.count -= 1;
                return true;
            }
            match link {
                Some(node) => link = &mut node.next,
                None => return false,
            }
        }
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}
