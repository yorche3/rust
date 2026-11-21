// data_structures_advanced — Node, TreeNode, BinaryTree, GraphNode y Graph.
//
// Especificación: 07_Data_Structures_Advanced
//
// Contrato y esqueletos del paso 4b. El contrato vive en la API pública del
// propio módulo (Rust no pide un archivo de contrato aparte: `lib.rs` exporta
// el submódulo y `mod.rs` reexporta los tipos públicos), un archivo por tipo,
// como en la base homologada `data_structures_basics`.
//
// Nombres: los del idioma (`new`, `value`, `left`, `right`, `set_left`,
// `set_right`, `insert`, `contains`, `is_empty`, `size`, `add_vertex`,
// `add_edge`, `has_edge`, `neighbors`, `vertex_count`, `edge_count`); Rust
// omite el prefijo `get_` en los accesos, igual que `Node::value` en la base.
//
// Indicador natural de ausencia: `Option`. La tabla de nombres canónicos fija
// para Rust `absent = None` y `present = Some(x)`, la fila de Fase 1 de
// `docs/ruleset/PSEUDO_CONVENTION.md` admite `Option/Maybe` como indicador de
// fallo, y `docs/AGENT_Template.md` (regla 3) aclara que la restricción no
// prohíbe los opcionales nativos ni los enlaces opcionales internos. Rust no
// tiene `null` en un tipo de valor, así que `Option` **es** el indicador de la
// casa. El éxito y el fallo de las operaciones booleanas son `true`/`false`.
//
// `Graph::neighbors` devuelve `Option<Vec<i32>>`: `Some(vacío)` es el vértice
// aislado y `None` el vértice inexistente, tal como exige la especificación.
// No se inventa un centinela.
//
// Adaptaciones declaradas:
// - Valor embebido en lugar de herencia: Rust no tiene herencia de datos, así
//   que `TreeNode` y `GraphNode` contienen un `Node` (`value: Node`) y cada
//   hijo valida sus propios enlaces.
// - Enlaces internos de la estructura como `Option<Box<…>>` (subárbol y
//   registro canónico); ninguna lectura pública devuelve `Option<i32>` salvo
//   `neighbors`, que necesita distinguir ausencia de vecinos vacíos.

mod binary_tree;
mod graph;
mod graph_node;
mod node;
mod tree_node;

pub use binary_tree::BinaryTree;
pub use graph::Graph;
pub use node::Node;
pub use tree_node::TreeNode;
