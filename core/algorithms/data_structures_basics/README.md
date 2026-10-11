# Data Structures Basics — Rust

Implementación de la especificación [06_Data_Structures_Basics](https://yorche3.github.io/programming_languages/core/algorithms/06_Data_Structures_Basics/) en **Rust**, utilizando un enfoque manual y minimalista con gestión de memoria segura mediante punteros en heap (`Box<Node>`) sin necesidad de bloques `unsafe`, conteo de referencias con `Rc<RefCell<T>>` ni bibliotecas de colecciones externas.

Define un tipo `Node` compartido y tres tipos de datos abstractos independientes: `LinkedList`, `Stack` y `Queue`, implementados manualmente desde cero sobre cadenas de nodos enlazados.

Implementation of the [06_Data_Structures_Basics](https://yorche3.github.io/programming_languages/core/algorithms/06_Data_Structures_Basics/) specification in **Rust**, using a manual and minimalist approach with safe memory management through heap pointers (`Box<Node>`) without requiring `unsafe` blocks, reference counting with `Rc<RefCell<T>>`, or external collection libraries.

Defines a shared `Node` type and three independent abstract data types: `LinkedList`, `Stack`, and `Queue`, implemented manually from scratch over chains of linked nodes.

---

## 📂 Archivos y estructura / Files & Structure

| Archivo / Directory | Propósito / Purpose |
|---|---|
| [`Cargo.toml`](Cargo.toml) | Manifiesto del paquete Cargo (edición 2024, sin dependencias externas). / Cargo package manifest (2024 edition, no external dependencies). |
| [`src/lib.rs`](src/lib.rs) | Raíz de la biblioteca que reexporta el módulo `data_structures_basics`. / Library root re-exporting the `data_structures_basics` module. |
| [`src/data_structures_basics/mod.rs`](src/data_structures_basics/mod.rs) | Módulo principal que declara los submódulos y reexporta `Node`, `LinkedList`, `Stack` y `Queue`. / Main module declaring submodules and re-exporting `Node`, `LinkedList`, `Stack`, and `Queue`. |
| [`src/data_structures_basics/node.rs`](src/data_structures_basics/node.rs) | Celda enlazada compartida con valor `i32` y puntero `Option<Box<Node>>`. / Shared linked cell with `i32` value and `Option<Box<Node>>` link. |
| [`src/data_structures_basics/linked_list.rs`](src/data_structures_basics/linked_list.rs) | Lista enlazada simple construida sobre `Node`. / Singly linked list built upon `Node`. |
| [`src/data_structures_basics/stack.rs`](src/data_structures_basics/stack.rs) | Pila LIFO construida sobre `Node`. / LIFO stack built upon `Node`. |
| [`src/data_structures_basics/queue.rs`](src/data_structures_basics/queue.rs) | Cola FIFO construida sobre `Node`. / FIFO queue built upon `Node`. |
| [`tests/data_structures_basics_tests.rs`](tests/data_structures_basics_tests.rs) | Suite de pruebas de integración con 22 tests unitarios que verifican las operaciones sucesivas. / Integration test suite with 22 unit tests validating successive operations. |
| [`.gitignore`](.gitignore) | Archivos generados por Cargo excluidos del repositorio (`target/`). / Cargo-generated build artifacts excluded from version control (`target/`). |

**Estructura de directorios / Directory structure:**

```text
data_structures_basics/
├── Cargo.toml
├── .gitignore
├── src/
│   ├── lib.rs
│   └── data_structures_basics/
│       ├── mod.rs
│       ├── node.rs
│       ├── linked_list.rs
│       ├── stack.rs
│       └── queue.rs
├── tests/
│   └── data_structures_basics_tests.rs
└── README.md
```

> **Nota de desviación / Deviation note:** La especificación propone un único archivo `src/data_structures_basics.ext`, `test/data_structures_basics_test.ext` y `test/run_tests.ext`. En Rust se adopta la convención idiomática modular de Cargo: `src/data_structures_basics/` divide las estructuras en submódulos internos manteniendo una API pública unificada, la suite reside en `tests/data_structures_basics_tests.rs` y `cargo test` gestiona el descubrimiento y ejecución de pruebas sin requerir un script `run_tests` separado.
>
> **Deviation note:** The specification proposes a single file `src/data_structures_basics.ext`, `test/data_structures_basics_test.ext`, and `test/run_tests.ext`. Rust adopts Cargo's idiomatic modular convention: `src/data_structures_basics/` splits structures into internal submodules while keeping a unified public API, tests reside in `tests/data_structures_basics_tests.rs`, and `cargo test` handles discovery and execution without requiring a separate `run_tests` script.

---

## 🛠️ Enfoque y construcción / Approach & Build

**ES:** El módulo se implementa de forma manual y minimalista utilizando las garantías de propiedad y préstamo de Rust. Cada celda enlazada `Node` es propietaria única de su sucesor en el heap mediante `Option<Box<Node>>`. Para evitar la complejidad de punteros compartidos con conteo de referencias (`Rc<RefCell<T>>`) o memoria no segura (`unsafe`), las estructuras son de enlace simple. `LinkedList` y `Queue` recorren la cadena hasta el final para inserciones al fondo (`insert_tail` y `enqueue`), manteniendo semántica estricta y seguridad de memoria garantizada en tiempo de compilación.

**EN:** The module is implemented manually and minimalistically using Rust's ownership and borrowing guarantees. Each linked cell `Node` exclusively owns its heap successor via `Option<Box<Node>>`. To avoid the complexity of shared reference-counted pointers (`Rc<RefCell<T>>`) or unsafe memory (`unsafe`), structures use single-ownership chains. `LinkedList` and `Queue` traverse the chain to the end for tail insertions (`insert_tail` and `enqueue`), preserving strict semantics and compile-time guaranteed memory safety.

### Inicialización / Initialization

```bash
cargo init --lib
```

---

## 📄 Configuración clave / Key Configuration

### `Cargo.toml` — paquete de biblioteca

```toml
[package]
name = "data_structures_basics"
version = "0.1.0"
edition = "2024"

[dependencies]
```

---

## 🚀 Compilación y ejecución / Build & Run

### Requisitos / Requirements

- **Rust toolchain** (probado con `rustc 1.98.1` y `cargo 1.98.1`).

### Verificación estática y pruebas / Static check & tests

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```

**Salida real / Actual output:**

```text
$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
```

```text
$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/data_structures_basics-a8c0f2b3a772a15c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/data_structures_basics_tests.rs (target/debug/deps/data_structures_basics_tests-bcc2deb6ac902787)

running 22 tests
test linked_list_delete ... ok
test linked_list_head ... ok
test linked_list_insert_head ... ok
test linked_list_insert_tail ... ok
test linked_list_is_empty ... ok
test linked_list_new ... ok
test linked_list_size ... ok
test node_get_next ... ok
test node_get_value ... ok
test node_set_next ... ok
test queue_dequeue ... ok
test queue_enqueue ... ok
test queue_is_empty ... ok
test queue_new ... ok
test queue_peek ... ok
test queue_size ... ok
test stack_is_empty ... ok
test stack_new ... ok
test stack_peek ... ok
test stack_pop ... ok
test stack_push ... ok
test stack_size ... ok

test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests data_structures_basics

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 🧠 Algoritmos y operaciones / Algorithms & Operations

| Operación / Operation | Entrada → salida / Input → output | Complejidad / Complexity | Notas / Notes |
|---|---|---|---|
| `Node::new` | `i32 → Node` | $O(1)$ | Constructor; celda con `value` y `next = None`. / Constructor; cell with `value` and `next = None`. |
| `Node::value` | `&self → i32` | $O(1)$ | Devuelve el entero almacenado en la celda. / Returns the integer stored in the cell. |
| `Node::next` | `&self → Option<&Node>` | $O(1)$ | Retorna referencia inmutable al nodo enlazado o `None`. / Returns immutable reference to linked node or `None`. |
| `Node::set_next` | `&mut self, Option<Box<Node>> → ()` | $O(1)$ | Reemplaza el enlace siguiente. / Replaces next link. |
| `LinkedList::new` | `() → LinkedList` | $O(1)$ | Constructor; lista vacía con `head = None` y `count = 0`. / Constructor; empty list with `head = None` and `count = 0`. |
| `LinkedList::is_empty` | `&self → bool` | $O(1)$ | Informa si `count == 0`. / Reports whether `count == 0`. |
| `LinkedList::size` | `&self → usize` | $O(1)$ | Retorna el número de nodos. / Returns node count. |
| `LinkedList::head` | `&self → i32` | $O(1)$ | Valor de cabeza o centinela `-1` si está vacía. / Head value or `-1` sentinel if empty. |
| `LinkedList::insert_head` | `&mut self, i32 → ()` | $O(1)$ | Inserta una nueva celda al inicio de la lista. / Inserts a new cell at the front. |
| `LinkedList::insert_tail` | `&mut self, i32 → ()` | $O(n)$ | Recorre la cadena de propiedad única e inserta al final. / Traverses single-ownership chain and appends at tail. |
| `LinkedList::delete` | `&mut self, i32 → bool` | $O(n)$ | Elimina la primera aparición de un valor; `true` si lo borró, `false` si no estaba. / Deletes first occurrence; `true` if removed, `false` if absent. |
| `Stack::new` | `() → Stack` | $O(1)$ | Constructor; pila vacía con `top = None` y `count = 0`. / Constructor; empty stack with `top = None` and `count = 0`. |
| `Stack::is_empty` | `&self → bool` | $O(1)$ | Informa si `count == 0`. / Reports whether `count == 0`. |
| `Stack::size` | `&self → usize` | $O(1)$ | Retorna el número de nodos. / Returns node count. |
| `Stack::push` | `&mut self, i32 → ()` | $O(1)$ | Coloca un nuevo nodo sobre `top`. / Places a new node on `top`. |
| `Stack::peek` | `&self → i32` | $O(1)$ | Observa el valor en `top` o `-1` si está vacía. / Inspects value at `top` or `-1` if empty. |
| `Stack::pop` | `&mut self → i32` | $O(1)$ | Extrae el nodo superior en orden LIFO o `-1` si está vacía. / Pops top node in LIFO order or `-1` if empty. |
| `Queue::new` | `() → Queue` | $O(1)$ | Constructor; cola vacía con `front = None` y `count = 0`. / Constructor; empty queue with `front = None` and `count = 0`. |
| `Queue::is_empty` | `&self → bool` | $O(1)$ | Informa si `count == 0`. / Reports whether `count == 0`. |
| `Queue::size` | `&self → usize` | $O(1)$ | Retorna el número de nodos. / Returns node count. |
| `Queue::enqueue` | `&mut self, i32 → ()` | $O(n)$ | Recorre la cadena e inserta al final de la cola. / Traverses chain and appends at end of queue. |
| `Queue::peek` | `&self → i32` | $O(1)$ | Observa el valor al frente o `-1` si está vacía. / Inspects value at front or `-1` if empty. |
| `Queue::dequeue` | `&mut self → i32` | $O(1)$ | Extrae el nodo al frente en orden FIFO o `-1` si está vacía. / Dequeues front node in FIFO order or `-1` if empty. |

---

## 🧩 Decisiones de diseño / Design decisions

| Decisión / Decision | Alternativa considerada / Alternative | Razón / Reason |
|---|---|---|
| Punteros en heap de propiedad única `Option<Box<Node>>` sin puntero a cola | Punteros compartidos con `Rc<RefCell<Node>>` o bloques `unsafe` con punteros crudos | Rust prohíbe múltiples punteros mutables a una misma estructura sin sincronización o interior mutability. Mantener una cadena simple de propiedad única (`Box<Node>`) garantiza seguridad total en código seguro (safe Rust) sin sobrecoste de wrappers en tiempo de ejecución. |
| Constructores `new()` e implementación de `Default` | Métodos `init()` mutadores sobre instancias no inicializadas | En Rust los tipos deben tener todos sus campos inicializados al construirse; la función asociada `new()` es la convención canónica para inicializar instancias vacías. |
| Centinela entero `-1` para operaciones de fallo (`head`, `peek`, `pop`, `dequeue`) | Tipos `Option<i32>` o `Result<i32, ...>` | La fase y el contrato de pruebas restringen los indicadores a enteros no ambiguos para alinear las pruebas con el pseudocódigo; solo los enlaces de nodos usan `Option`. |
| `Node::value` inmutable tras la construcción | Método mutador `set_value` | La especificación no requiere mutar el valor de una celda tras su inicialización; mantenerlo inmutable refuerza la seguridad de datos. |

---

## 🔀 Adaptaciones idiomáticas / Idiomatic adaptations

| Especificación / Specification | Adaptación / Adaptation | Justificación / Justification |
|---|---|---|
| `Node.init(value)` y `ADT.init()` | Funciones asociadas `new(value)` y `new()` | Convención canónica en Rust para constructores de instancias. |
| `Node.get_value()` y `get_next()` | Métodos `value(&self)` y `next(&self)` | Naming idiomático en Rust (omisión de prefijo `get_` en getters). |
| `Node.set_next` devuelve `this` en pseudocódigo | `set_next(&mut self, next: Option<Box<Node>>)` devuelve `()` | Las funciones mutadoras *in-place* retornan convencionalmente la tupla unitaria `()` en Rust. |
| Complejidad $O(1)$ para `insert_tail` y `enqueue` | Complejidad $O(n)$ mediante recorrido mutable | Con propiedad única pura (`Box<Node>`) y sin `unsafe` ni `Rc<RefCell>`, no es posible mantener un puntero mutable independiente al final de la cadena de forma segura. La semántica funcional observable se conserva íntegra. |
| Ausencia de enlace (`absent`) | `Option::None` sobre `Option<Box<Node>>` y `Option<&Node>` | Representación nativa y tipada en Rust para ausencia de valor/enlace. |
| `LinkedList.get_head()` | Método `head(&self)` | Naming idiomático en Rust; devuelve `-1` si la lista está vacía. |
| Layout de archivos `src/` modular | Submódulos organizados en `src/data_structures_basics/` | Separación limpia de responsabilidades respetando la estructura de paquetes de Cargo. |

---

## 🚨 Indicadores de fallo / Failure indicators

| Operación / Operation | Situación de fallo / Failure situation | Indicador / Indicator | Ejemplo / Example |
|---|---|---|---|
| `Node::next(&self)` | Nodo sin enlace siguiente / Node has no next link | `None` | `node.next().is_none() == true` |
| `LinkedList::head(&self)` | Lista vacía (`count == 0`) / Empty list | `-1` | `list.head() == -1` tras `new()` |
| `LinkedList::delete(&mut self, val)` | Valor no presente en la lista / Value absent | `false` | `list.delete(99) == false` |
| `Stack::peek(&self)` | Pila vacía (`count == 0`) / Empty stack | `-1` | `stack.peek() == -1` tras `new()` |
| `Stack::pop(&mut self)` | Pila vacía (`count == 0`) / Empty stack | `-1` | `stack.pop() == -1` tras `new()` |
| `Queue::peek(&self)` | Cola vacía (`count == 0`) / Empty queue | `-1` | `queue.peek() == -1` tras `new()` |
| `Queue::dequeue(&mut self)` | Cola vacía (`count == 0`) / Empty queue | `-1` | `queue.dequeue() == -1` tras `new()` |

---

## ✅ Cobertura de pruebas / Test coverage

| Caso de la especificación / Specification case | Cubierto / Covered | Prueba / Test | Notas / Notes |
|---|---|:--:|---|
| Node — Inicializar y observar valor/enlace | Sí | `tests/data_structures_basics_tests.rs:21` (`node_get_value`, `node_get_next`) | Verifica `value() == 42` y enlace `None`. |
| Node — Inicializar otro nodo, enlazar y recorrer | Sí | `tests/data_structures_basics_tests.rs:40` (`node_set_next`) | Enlaza segundo nodo (valor 7), recorre y verifica desvinculación a `None`. |
| LinkedList — Estado vacío | Sí | `tests/data_structures_basics_tests.rs:60` (`linked_list_new`) | Valida `size() == 0`, `is_empty() == true`, `head() == -1`. |
| LinkedList — Insertar por ambos extremos | Sí | `tests/data_structures_basics_tests.rs:75` (`linked_list_head`, `linked_list_insert_head`, `linked_list_insert_tail`) | Inserciones sucesivas combinando cabeza y cola; valida tamaño y cabeza visible. |
| LinkedList — Eliminar primera aparición | Sí | `tests/data_structures_basics_tests.rs:178` (`linked_list_delete`) | Elimina cabeza, cola y elementos intermedios verificando actualización de tamaño y retorno `true`. |
| LinkedList — Valor ausente | Sí | `tests/data_structures_basics_tests.rs:178` (`linked_list_delete`) | Intento de borrado de `99`; verifica retorno `false` y tamaño inalterado. |
| LinkedList — Vaciar | Sí | `tests/data_structures_basics_tests.rs:178` (`linked_list_delete`) | Vaciado secuencial hasta `size() == 0`, `is_empty() == true`, `head() == -1` y reutilización exitosa. |
| Stack — Estado vacío y extracción fallida | Sí | `tests/data_structures_basics_tests.rs:330` (`stack_new`, `stack_pop`) | Valida `is_empty() == true`, `size() == 0`, `peek() == -1`, `pop() == -1`. |
| Stack — LIFO y `peek` no mutante | Sí | `tests/data_structures_basics_tests.rs:342` (`stack_push`, `stack_peek`) | Apila 1, 2, 3; verifica `peek() == 3` sin mutar el tamaño. |
| Stack — Extracción y reutilización | Sí | `tests/data_structures_basics_tests.rs:362` (`stack_pop`) | Secuencia LIFO completa (3, 2, 1), vaciado total y reutilización posterior. |
| Stack — Vacío tras extracción | Sí | `tests/data_structures_basics_tests.rs:362` (`stack_pop`) | `pop()` sobre pila vacía devuelve `-1` y preserva `is_empty() == true`. |
| Queue — Estado vacío y extracción fallida | Sí | `tests/data_structures_basics_tests.rs:466` (`queue_new`, `queue_dequeue`) | Valida `is_empty() == true`, `size() == 0`, `peek() == -1`, `dequeue() == -1`. |
| Queue — FIFO y `peek` no mutante | Sí | `tests/data_structures_basics_tests.rs:478` (`queue_enqueue`, `queue_peek`) | Encola 1, 2, 3; verifica `peek() == 1` sin mutar el tamaño. |
| Queue — Extracción y reutilización | Sí | `tests/data_structures_basics_tests.rs:506` (`queue_dequeue`) | Secuencia FIFO completa (1, 2, 3), vaciado total y encolado/desencolado posterior. |
| Queue — Vacío tras extracción | Sí | `tests/data_structures_basics_tests.rs:506` (`queue_dequeue`) | `dequeue()` sobre cola vacía devuelve `-1` y preserva `is_empty() == true`. |

---

## ⚠️ Limitaciones conocidas / Known limitations

| Limitación / Limitation | Impacto / Impact | Alternativa o plan / Workaround or plan |
|---|---|---|
| Dominio restringido a enteros no negativos / centinela `-1` | Si se almacena `-1`, no puede distinguirse de la indicación de fallo en `head`, `peek`, `pop` o `dequeue` | Definido por la especificación para mantener homogeneidad sin usar `Option`/`Result` en valores enteros en esta fase; los casos de prueba usan enteros positivos. |
| Inserción al final $O(n)$ en `LinkedList` y `Queue` | Inserción en cola degrada linealmente con el tamaño de la colección | Para lograr $O(1)$ sin comprometer la seguridad de Rust se requeriría `unsafe` o `Rc<RefCell<>>`, los cuales se descartan intencionalmente en este módulo elemental. |

---

## 📝 Notas de implementación / Implementation Notes

### 🧱 Nodo compartido y estructuras independientes / Shared node and independent structures

`Node` es el único tipo de celda enlazada declarado en el módulo (`src/data_structures_basics/node.rs`). Las tres estructuras (`LinkedList`, `Stack` y `Queue`) se implementan de forma autónoma gestionando directamente sus propios enlaces (`head`, `top`, `front`) y contadores de tamaño. `Stack` y `Queue` no envuelven a `LinkedList` ni utilizan colecciones de la biblioteca estándar (`std::collections::LinkedList`, `std::collections::VecDeque`).

`Node` is the single linked cell type declared in the module (`src/data_structures_basics/node.rs`). All three structures (`LinkedList`, `Stack`, and `Queue`) are implemented autonomously, directly managing their own links (`head`, `top`, `front`) and size counters. Neither `Stack` nor `Queue` wrap `LinkedList` or use standard library collections (`std::collections::LinkedList`, `std::collections::VecDeque`).

### 💾 Gestión de memoria y desasignación / Memory management and deallocation

Rust gestiona la memoria dinámica mediante el trait `Drop` y la propiedad estricta de `Box<Node>`. Cuando una estructura (`LinkedList`, `Stack`, `Queue`) o celda sale de ámbito, el compilador desasigna automáticamente la memoria asignada en el heap sin fugas. Al extraer un elemento con `pop()` o `dequeue()`, se utiliza `take()` sobre el enlace opcional para tomar propiedad del nodo de la cabecera, reasignar el puntero al sucesor y permitir que la celda extraída se libere al salir de la función. En `delete()` de `LinkedList`, el préstamo inmutable se desacopla del reenganche mutable permitiendo eliminar nodos intermedios de forma segura.

Rust manages dynamic heap memory through the `Drop` trait and strict ownership of `Box<Node>`. When a structure or cell goes out of scope, the compiler automatically frees heap-allocated memory with no memory leaks. When popping or dequeuing with `pop()` or `dequeue()`, `take()` is used on the optional link to take ownership of the head node, advance the pointer to its successor, and drop the extracted cell when leaving function scope. In `LinkedList::delete()`, immutable borrowing is separated from mutable link updating, allowing intermediate nodes to be spliced out safely.

### 🌐 Otras implementaciones / Other implementations

Este proyecto también está implementado en otros lenguajes. Explora el repositorio principal para consultar las demás versiones.

This project is also implemented in other languages. Explore the main repository to see the other versions.

---

## 🔍 Checklist de validación / Validation checklist

- [x] La suite nativa se ejecutó y su salida real está copiada en este README.
- [x] Cada caso de la especificación tiene su fila en _Cobertura de pruebas_ (o `Omitido` con razón).
- [x] Cada desviación del pseudocódigo o de la ubicación esperada está en _Adaptaciones idiomáticas_.
- [x] Cada operación con fallo posible está en _Indicadores de fallo_.
- [x] No hay rutas absolutas del autor, credenciales ni salidas inventadas.
- [x] Los enlaces relativos resuelven dentro del repositorio y el documento es bilingüe.
- [x] Ninguna sección repite lo que ya dice la especificación.

---

## 📚 Referencias / References

| Tipo / Kind | Referencia / Reference |
|---|---|
| Especificación / Specification | [`06_Data_Structures_Basics.md`](../../../../../docs/core/algorithms/06_Data_Structures_Basics.md) |
| Módulo homologado del lenguaje / Homologated module | [`rust/core/foundations/numbers/`](../../foundations/numbers/README.md) |
| Guía de inicialización / Initialisation guide | [`core/00_Project_Initialization_Guide.md`](../../../../../docs/core/00_Project_Initialization_Guide.md) |
| Adaptaciones idiomáticas / Idiomatic adaptations | [`AGENT_Template.md`](../../../../../docs/AGENT_Template.md) |
| Validación de la documentación / Documentation validation | [`WORKFLOW.md`](../../../../../docs/WORKFLOW.md) |
| Documentación oficial del lenguaje / Language official docs | [The Rust Programming Language](https://doc.rust-lang.org/book/) |

---

*[← Volver a Algoritmos Puros](../README.md)*

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
