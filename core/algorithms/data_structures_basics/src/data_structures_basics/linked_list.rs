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

    /// Esqueleto: valor de la cabeza, o -1 cuando la lista está vacía.
    pub fn head(&self) -> i32 {
        -1
    }

    /// Esqueleto: true solo si no hay nodos.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Esqueleto: número de nodos.
    pub fn size(&self) -> usize {
        self.count
    }

    /// Esqueleto: inserta `value` al principio. O(1).
    pub fn insert_head(&mut self, _value: i32) {}

    /// Esqueleto: inserta `value` al final. O(n) — ver la nota del módulo.
    pub fn insert_tail(&mut self, _value: i32) {}

    /// Esqueleto: borra la primera aparición de `value`; `true` si borró y
    /// `false` si el valor no estaba. O(n).
    pub fn delete(&mut self, _value: i32) -> bool {
        false
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}
