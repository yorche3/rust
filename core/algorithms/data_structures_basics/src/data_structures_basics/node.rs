// Node: celda enlazada compartida por LinkedList, Stack y Queue.
//
// Adecuación: `value` no se reasigna después de la construcción; `next` sí.

/// Celda enlazada compartida por `LinkedList`, `Stack` y `Queue`.
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    /// Crea una celda con `value` y sin enlace (el `init(value)` de la
    /// especificación).
    pub fn new(value: i32) -> Self {
        Self { value, next: None }
    }

    /// Esqueleto: valor de la celda.
    pub fn value(&self) -> i32 {
        -1
    }

    /// Esqueleto: enlace siguiente, o `None` cuando está ausente.
    pub fn next(&self) -> Option<&Node> {
        None
    }

    /// Esqueleto: reemplaza el enlace siguiente.
    pub fn set_next(&mut self, _next: Option<Box<Node>>) {}
}
