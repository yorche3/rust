// Node: celda enlazada compartida por LinkedList, Stack y Queue.
//
// Adecuación: `value` no se reasigna después de la construcción; `next` sí.

/// Celda enlazada compartida por `LinkedList`, `Stack` y `Queue`.
pub struct Node {
    value: i32,
    // Visible para los módulos hermanos (`linked_list`, `stack`, `queue`),
    // que recorren y reenganchan la cadena.
    pub(super) next: Option<Box<Node>>,
}

impl Node {
    /// Crea una celda con `value` y sin enlace (el `init(value)` de la
    /// especificación).
    pub fn new(value: i32) -> Self {
        Self { value, next: None }
    }

    /// Valor de la celda.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Enlace siguiente, o `None` cuando está ausente.
    pub fn next(&self) -> Option<&Node> {
        self.next.as_deref()
    }

    /// Reemplaza el enlace siguiente.
    pub fn set_next(&mut self, next: Option<Box<Node>>) {
        self.next = next;
    }
}
