// data_structures_basics — Node, LinkedList, Stack y Queue.
//
// Especificación: 06_Data_Structures_Basics
//
// Contrato: un único `Node` compartido y tres estructuras construidas desde
// cero sobre él. Cada estructura nace vacía con `new()` (el `init()` de la
// especificación) y no tiene capacidad.
//
// Adecuaciones de salida de esta fase: las lecturas enteras que pueden fallar
// devuelven el centinela **-1** (`head`, `peek`, `pop`, `dequeue`); las
// booleanas devuelven `true`/`false` (`delete`, `is_empty`) y `size` devuelve
// el número de nodos. **Solo los enlaces** usan `Option` (`Node::next`), que es
// la representación idiomática del enlace ausente: nada devuelve
// `Option<i32>`. No se entra en pánico.
//
// Desviación de complejidad declarada: `LinkedList::insert_tail` y
// `Queue::enqueue` son O(n) en esta implementación. Con una sola cadena de
// `Box<Node>` y sin `unsafe`, `Rc<RefCell<>>` ni arena, el apéndice O(1) no es
// posible; la semántica se conserva y solo cambia la cota.
//
// Implementación del paso 5: recorridos propios sobre la cadena de `Node`, sin
// `unsafe`, sin `Rc<RefCell<>>` y sin bibliotecas de estructuras.

mod linked_list;
mod node;
mod queue;
mod stack;

pub use linked_list::LinkedList;
pub use node::Node;
pub use queue::Queue;
pub use stack::Stack;
