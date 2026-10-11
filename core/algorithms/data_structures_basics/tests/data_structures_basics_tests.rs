// Casos de prueba de la especificación 06_Data_Structures_Basics.md
//
// Contrato: cuatro tipos públicos (`Node`, `LinkedList`, `Stack`, `Queue`) y un
// único `Node` compartido. Cada tabla de la especificación se ejecuta como una
// secuencia de operaciones sobre la MISMA instancia, sin recrear objetos para
// cada borde: los escenarios van de lo vacío a lo lleno y vuelven a lo vacío.
//
// Adecuaciones de salida de la fase: las lecturas enteras que pueden fallar
// devuelven -1 (`head`, `peek`, `pop`, `dequeue`); las booleanas devuelven
// `true`/`false` (`delete`, `is_empty`) y `size` devuelve el número de nodos.
// Solo los enlaces son `Option` (`Node::next`), así que la ausencia se
// comprueba con `is_none()`. No se espera ningún pánico.

use data_structures_basics::data_structures_basics::{LinkedList, Node, Queue, Stack};

