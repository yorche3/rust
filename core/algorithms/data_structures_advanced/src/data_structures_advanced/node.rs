// Node: base local del módulo. Solo el valor, sin enlaces y sin validaciones.
//
// No se instancia por sí sola: `TreeNode` y `GraphNode` la reutilizan por
// valor embebido (`value: Node`), que es la forma que admite el contrato
// cuando el lenguaje no tiene herencia de datos.

/// Base local del módulo: solo el valor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    value: i32,
}

impl Node {
    /// Crea la base con `value` (el `init(value)` de la especificación).
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Valor embebido.
    pub fn value(&self) -> i32 {
        self.value
    }
}
