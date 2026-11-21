# Data Structures Advanced — Rust

Implementación de la especificación
[07_Data_Structures_Advanced](https://yorche3.github.io/programming_languages/core/algorithms/07_Data_Structures_Advanced/)
en **Rust**, con un crate de biblioteca generado por `cargo new --lib` y pruebas
de integración ejecutadas por `cargo test`.

**ES:** Un `Node` que solo guarda el valor, un `TreeNode` con los enlaces
`left`/`right`, un `BinaryTree` con raíz propia y contador de valores distintos,
un `GraphNode` que encadena el registro canónico de vértices y un `Graph` con
contadores separados. El registro de vértices, el descenso del ABB, la detección
de duplicados y la inserción ordenada de vecinos se escriben a mano: no se usa
`HashMap`, `BTreeMap`, `HashSet` ni las rutinas de búsqueda u ordenación de la
biblioteca estándar.

**EN:** A `Node` holding only the value, a `TreeNode` with `left`/`right` links,
a `BinaryTree` with its own root and a distinct-value counter, a `GraphNode` that
links the canonical vertex registry, and a `Graph` with separate counters. The
vertex registry, the BST descent, duplicate detection and ordered neighbour
insertion are written by hand: no `HashMap`, `BTreeMap`, `HashSet` or library
search/sort routines are used.

---

## 📂 Archivos y estructura / Files & Structure

**ES:** El layout es el de Cargo: la biblioteca en `src/` y la suite como
*crate* de pruebas de integración en `tests/`. El contrato de la especificación
no pide un archivo aparte —Rust declara la API pública del propio módulo—, así
que cada tipo ocupa su archivo dentro del submódulo y `lib.rs` solo lo exporta,
igual que en la base `data_structures_basics`.

**EN:** The layout is Cargo's: the library under `src/` and the suite as an
integration test crate under `tests/`. The specification does not ask for a
separate contract file — Rust declares the module's public API itself — so each
type gets its own file inside the submodule and `lib.rs` only exports it, as in
the `data_structures_basics` base.

| Archivo / Directorio | Propósito / Purpose |
|---|---|
| `src/lib.rs` | Raíz del crate: declara `pub mod data_structures_advanced`. / Crate root: declares `pub mod data_structures_advanced`. |
| `src/data_structures_advanced/mod.rs` | Cabecera del módulo y reexportación de los tipos públicos; documenta las decisiones de contrato y las adaptaciones. / Module header and re-exports of the public types; documents the contract decisions and adaptations. |
| `src/data_structures_advanced/node.rs` | `Node`: solo el valor, sin enlaces ni validaciones. / `Node`: value only, no links, no validations. |
| `src/data_structures_advanced/tree_node.rs` | `TreeNode`: valor embebido, `left`/`right` y acceso mutable interno para el árbol. / `TreeNode`: embedded value, `left`/`right` and internal mutable access for the tree. |
| `src/data_structures_advanced/binary_tree.rs` | `BinaryTree`: raíz, contador y descenso por la invariante del ABB. / `BinaryTree`: root, counter and descent by the BST invariant. |
| `src/data_structures_advanced/graph_node.rs` | `GraphNode`: vértice interno con vecinos ordenados y enlace al siguiente vértice. / `GraphNode`: internal vertex with sorted neighbours and the next-vertex link. |
| `src/data_structures_advanced/graph.rs` | `Graph`: registro canónico, contadores y las operaciones del contrato. / `Graph`: canonical registry, counters and the contract operations. |
| `tests/data_structures_advanced_tests.rs` | Suite de integración: 23 casos y cinco ayudantes compartidos. / Integration suite: 23 cases and five shared helpers. |
| `Cargo.toml` | Manifiesto del crate; `edition = "2024"` y sin dependencias. / Crate manifest; `edition = "2024"`, no dependencies. |
| `Cargo.lock` | Resolución de dependencias (vacía: no hay ninguna). / Dependency resolution (empty: there are none). |
| `.gitignore` | Ignora `target/`. / Ignores `target/`. |

**Desviación de ubicación / Location deviation:** la especificación propone
`src/{modulo}.ext`; aquí el código vive en `src/data_structures_advanced/` (un
archivo por tipo) y la suite en `tests/`, que es donde Cargo compila un *crate*
de pruebas de integración contra la API pública. / The specification proposes
`src/{module}.ext`; here the code lives in `src/data_structures_advanced/` (one
file per type) and the suite in `tests/`, which is where Cargo builds an
integration test crate against the public API.

---

## 🛠️ Enfoque y construcción / Approach & Build

**ES:** El crate se creó con `cargo new --lib` y se ajustó a mano: `src/lib.rs`
exporta el submódulo y cada tipo se separó en su archivo. No hay dependencias
externas ni *features*; el proyecto se compila y se prueba con las herramientas
del propio toolchain. El módulo gemelo `data_structures_basics` de este
submódulo usó el mismo layout, así que la suite se apoya en la API pública del
crate y no en rutas internas.

**EN:** The crate was created with `cargo new --lib` and adjusted by hand:
`src/lib.rs` exports the submodule and each type was split into its own file.
There are no external dependencies or features; the project builds and tests
with the toolchain alone. This submodule's sibling `data_structures_basics` used
the same layout, so the suite relies on the crate's public API rather than
internal paths.

---

## 📄 Configuración clave / Key Configuration

**ES:** `Cargo.toml` declara solo `[package]` —nombre, versión y
`edition = "2024"`— y una sección `[dependencies]` vacía. No hay perfiles
personalizados, `[dev-dependencies]` ni `[features]`: la suite usa `assert!`,
`assert_eq!` y `assert_ne!` del preludio. `cargo fmt --check` y
`cargo clippy --all-targets -- -D warnings` son las compuertas estáticas del
módulo.

**EN:** `Cargo.toml` declares only `[package]` —name, version and
`edition = "2024"`— plus an empty `[dependencies]` section. There are no custom
profiles, `[dev-dependencies]` or `[features]`: the suite uses `assert!`,
`assert_eq!` and `assert_ne!` from the prelude. `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` are the module's static gates.

---

## 🚀 Compilación y ejecución / Build & Run

```bash
cargo build                                  # compilación / build
cargo clippy --all-targets -- -D warnings    # compuerta estática / static gate
cargo fmt --check                            # formato / formatting
cargo test -- --test-threads=1               # suite / tests (orden fijo / fixed order)
```

**Salida real / Actual output:**

```text
$ cargo fmt --check
fmt: limpio

$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s

$ cargo test -- --test-threads=1
     Running unittests src/lib.rs (target/debug/deps/data_structures_advanced-1a97fb3fd9520c8b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/data_structures_advanced_tests.rs (target/debug/deps/data_structures_advanced_tests-a7981696d67eba4d)

running 23 tests
test binary_tree_create_empty ... ok
test binary_tree_idempotent_duplicate ... ok
test binary_tree_insert_lower_and_higher ... ok
test binary_tree_insert_root ... ok
test binary_tree_missing_value_in_nonempty_tree ... ok
test binary_tree_multilevel_branches ... ok
test binary_tree_ordered_insertions ... ok
test graph_create_empty ... ok
test graph_duplicate_edge ... ok
test graph_duplicate_vertex_preserves_adjacency ... ok
test graph_isolated_vertex ... ok
test graph_missing_edge_query ... ok
test graph_missing_endpoint_without_mutation ... ok
test graph_neighbour_result_isolation ... ok
test graph_neighbours_of_missing_vertex ... ok
test graph_nonconsecutive_labels_and_directed_edge ... ok
test graph_self_loop_and_duplicate ... ok
test graph_sorted_neighbours_and_counts ... ok
test tree_node_absent_initial_links ... ok
test tree_node_create ... ok
test tree_node_link_two_children ... ok
test tree_node_traverse_each_link ... ok
test tree_node_unlink_children ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Acta del sprint / Sprint evidence record:
[`rust.md`](../../../../docs/evidence/algorithms/data_structures_advanced/rust.md).

---

## 🧠 Algoritmos y operaciones / Algorithms & Operations

**ES:** La complejidad usa `h` para la altura del árbol, `V` para los vértices
registrados y `d(u)` para los vecinos de un vértice. Los nombres son los del
idioma: Rust omite el prefijo `get_` en los accesos, como ya hace `Node::value`
en la base.

**EN:** Complexity uses `h` for the tree height, `V` for registered vertices and
`d(u)` for a vertex's neighbours. Names are the language's: Rust drops the `get_`
prefix on accessors, as `Node::value` already does in the base.

| Operación / Operation | Entrada → salida / Input → output | Complejidad / Complexity | Notas / Notes |
|---|---|---|---|
| `Node::new` | `i32 → Node` | `O(1)` | Solo el valor; no se instancia por sí solo. / Value only; not instantiated on its own. |
| `Node::value` | `&Node → i32` | `O(1)` | Copia el `i32`. / Copies the `i32`. |
| `TreeNode::new` | `i32 → TreeNode` | `O(1)` | Ambos enlaces `None`. / Both links `None`. |
| `TreeNode::value` | `&TreeNode → i32` | `O(1)` | Delega en el `Node` embebido. / Delegates to the embedded `Node`. |
| `TreeNode::left`, `right` | `&TreeNode → Option<&TreeNode>` | `O(1)` | Enlace ausente = `None`. / Absent link = `None`. |
| `TreeNode::set_left`, `set_right` | `&mut TreeNode, Option<Box<TreeNode>> → ()` | `O(1)` | Muta el enlace en el sitio. / Mutates the link in place. |
| `BinaryTree::new` | `→ BinaryTree` | `O(1)` | Raíz `None` y contador 0. / `None` root, counter 0. |
| `BinaryTree::insert` | `&mut BinaryTree, i32 → bool` | `O(h)` | Éxito siempre; un duplicado no crece `size`. / Always succeeds; a duplicate does not grow `size`. |
| `BinaryTree::contains` | `&BinaryTree, i32 → bool` | `O(h)` | Descenso iterativo por la invariante del ABB. / Iterative descent by the BST invariant. |
| `BinaryTree::is_empty`, `size` | `&BinaryTree → bool` / `usize` | `O(1)` | Contador propio. / Own counter. |
| `GraphNode::new` | `i32 → GraphNode` | `O(1)` | Interno a `Graph`. / Internal to `Graph`. |
| `Graph::new` | `→ Graph` | `O(1)` | Registro `None` y ambos contadores en 0. / `None` registry, both counters at 0. |
| `Graph::add_vertex` | `&mut Graph, i32 → bool` | `O(V)` | Busca el duplicado; si no existe, antepone el vértice al registro. / Looks up the duplicate; when absent, prepends the vertex to the registry. |
| `Graph::add_edge` | `&mut Graph, i32, i32 → bool` | `O(V + d(u))` | Falla sin mutar si falta un extremo; la duplicada no cuenta. / Fails without mutation when an endpoint is missing; a duplicate is not counted. |
| `Graph::has_edge` | `&Graph, i32, i32 → bool` | `O(V + d(u))` | Recorre la adyacencia del origen. / Walks the source's adjacency. |
| `Graph::neighbors` | `&Graph, i32 → Option<Vec<i32>>` | `O(V + d(u))` | Copia independiente; `Some(vec![])` aislado, `None` inexistente. / Independent copy; `Some(vec![])` isolated, `None` missing. |
| `Graph::is_empty`, `vertex_count`, `edge_count` | `&Graph → bool` / `usize` | `O(1)` | Contadores separados. / Separate counters. |

---

## 🧩 Decisiones de diseño / Design decisions

| Decisión / Decision | Alternativa considerada / Alternative | Razón / Reason |
|---|---|---|
| Valor embebido (`value: Node`) en `TreeNode` y `GraphNode` | `trait` base con el valor, o `Deref` al `Node` | El contrato admite herencia o valor embebido; Rust no tiene herencia de datos y un `trait` obligaría a un acceso indirecto por cada lectura. El valor embebido mantiene `value()` en `O(1)` y el tipo base sigue existiendo y sin instanciarse. / The contract allows inheritance or an embedded value; Rust has no data inheritance and a trait would force an indirect access per read. The embedded value keeps `value()` at `O(1)` and the base type still exists, uninstantiated. |
| `Vec<i32>` para la adyacencia de `GraphNode` | `LinkedList<i32>` de la biblioteca estándar | La especificación permite la lista de biblioteca solo como almacenamiento; con `Vec` la inserción ordenada se escribe igual de manual (avance por comparación e `insert` por posición) y la copia de `neighbors` es un `to_vec()` directo. / The specification allows the library list only as storage; with `Vec` the ordered insertion is just as manual (advance by comparison, `insert` by position) and copying in `neighbors` is a direct `to_vec()`. |
| Registro de vértices enlazado por `next_vertex` (`Option<Box<GraphNode>>`) | `HashMap<i32, …>` o índice por etiqueta | La especificación prohíbe delegar el registro en mapas, conjuntos o índices: la búsqueda por etiqueta debe recorrerse. / The specification forbids delegating the registry to maps, sets or indexes: label lookup must be traversed. |
| Alta de vértice al principio del registro | Añadir al final recorriendo la cola | Anteponer es un enlace `O(1)` que no copia el registro; el orden interno del registro no es observable —los vecinos van ordenados por su cuenta y los contadores son propios— y el coste `O(V)` del contrato se mantiene en la búsqueda del duplicado. / Prepending is an `O(1)` link that does not copy the registry; the registry's internal order is unobservable — neighbours are sorted on their own and counters are separate — and the contract's `O(V)` cost remains in the duplicate lookup. |
| `Option<Vec<i32>>` como resultado de `neighbors` | Centinela `Vec` vacío con una bandera, o `-1` | El contrato exige distinguir «vértice inexistente» de «vértice aislado»; `Option` es la representación de ausencia que fija la tabla de nombres del lenguaje, y un centinela inventado estaría prohibido. / The contract requires telling "missing vertex" apart from "isolated vertex"; `Option` is the absence representation the language's names table fixes, and an invented sentinel is forbidden. |
| Suite en `tests/` como *crate* de integración | Módulo `#[cfg(test)]` dentro de `src/` | El crate de integración solo puede llamar a la API pública, así que las pruebas no dependen de campos ni ayudas internas; es el uso que Cargo documenta para probar la API de una biblioteca. / The integration crate can only call the public API, so tests depend on no internal fields or helpers; it is the use Cargo documents for testing a library's API. |
| `impl Default` para `BinaryTree` y `Graph` | Dejar solo `new()` | `clippy::new_without_default` es un error con `-D warnings`; `Default` delega en `new()`, no añade estado ni cambia el contrato. / `clippy::new_without_default` is an error under `-D warnings`; `Default` delegates to `new()` and adds no state or contract change. |

---

## 🔀 Adaptaciones idiomáticas / Idiomatic adaptations

| Especificación / Specification | Adaptación / Adaptation | Justificación / Justification |
|---|---|---|
| `init(value)`, `init()` | `Node::new`, `TreeNode::new`, `BinaryTree::new`, `Graph::new`, `GraphNode::new` | En Rust el constructor es una función asociada llamada `new`; el resto de la biblioteca estándar sigue esa convención. / In Rust the constructor is an associated function named `new`; the rest of the standard library follows that convention. |
| `get_value`, `get_left`, `get_right`, `get_neighbors` | `value`, `left`, `right`, `neighbors` | Rust omite el prefijo `get_` en los accesos; la base `data_structures_basics` ya usa `Node::value`. / Rust drops the `get_` prefix on accessors; the `data_structures_basics` base already uses `Node::value`. |
| `set_left(node)`, `set_right(node)` | `set_left(&mut self, Option<Box<TreeNode>>)` | La mutación es en el sitio y el enlace ausente se expresa con `None`; no hay variante inmutable que devolver. / Mutation is in place and an absent link is expressed with `None`; there is no immutable variant to return. |
| Éxito y fallo de `insert`, `add_vertex`, `add_edge`, `has_edge` | `true` / `false` | Las operaciones del contrato tienen una condición de éxito booleana; el tipo `bool` la expresa sin envoltorios. / The contract operations have a boolean success condition; `bool` expresses it without wrappers. |
| Ausencia de enlace y de vértice | `Option` (`None` / `Some(x)`) | `None`/`Some(x)` es el indicador que la tabla `canonical_names.tsv` fija para Rust y la Fase 1 admite; el lenguaje no tiene `null` en un tipo de valor. El caso no nulo de la entrada se cubre igual. / `None`/`Some(x)` is the indicator `canonical_names.tsv` fixes for Rust and Phase 1 admits; the language has no `null` in a value type. The non-null input case is still covered. |
| Caso «Adaptación inmutable (si aplica)» de `TreeNode` | **Omitido**: `set_left` muta el receptor | El caso solo aplica si la estructura es inmutable; aquí `set_left`/`set_right` cambian el nodo y devuelven `()`, así que no hay nodo nuevo que comparar con el original. Se declara en _Cobertura de pruebas_. / The case applies only to immutable structures; here `set_left`/`set_right` change the node and return `()`, so there is no new node to compare with the original. It is declared under _Test coverage_. |
| Ubicación esperada `src/{modulo}.ext` | `src/data_structures_advanced/` (un archivo por tipo) y suite en `tests/` | Cargo compila la biblioteca desde `src/lib.rs` y las pruebas de integración desde `tests/`; un archivo por tipo evita un módulo de mil líneas. / Cargo builds the library from `src/lib.rs` and integration tests from `tests/`; one file per type avoids a thousand-line module. |

---

## 🚨 Indicadores de fallo / Failure indicators

| Operación / Operation | Situación de fallo / Failure situation | Indicador / Indicator | Ejemplo / Example |
|---|---|---|---|
| `TreeNode::left`, `TreeNode::right` | Enlace ausente / Absent link | `None` | `TreeNode::new(10).left()` → `None` |
| `BinaryTree::contains` | Valor ausente del árbol / Value absent from the tree | `false` | `tree_with(&[50]).contains(99)` → `false` |
| `BinaryTree::insert` | Sin fallo contractual; el duplicado también es éxito / No contractual failure; a duplicate also succeeds | `true` | `insert(30)` dos veces → `true`, `size()` = 3 / twice → `true`, `size()` = 3 |
| `Graph::add_vertex` | Sin fallo contractual; el duplicado también es éxito / No contractual failure; a duplicate also succeeds | `true` | `add_vertex(10)` dos veces → `true`, `vertex_count()` = 1 / twice → `true`, `vertex_count()` = 1 |
| `Graph::add_edge` | Extremo `u` o `v` no registrado / Endpoint `u` or `v` not registered | `false` | `add_edge(10, 99)` → `false`, sin mutación / no mutation |
| `Graph::has_edge` | Arista ausente o extremo no registrado / Absent edge or unregistered endpoint | `false` | `has_edge(70, 10)` → `false` |
| `Graph::neighbors` | Vértice no registrado / Vertex not registered | `None`, distinto de `Some(vec![])` / distinct from `Some(vec![])` | `neighbors(99)` → `None`; `neighbors(10)` aislado → `Some(vec![])` |
| Entrada nula o inválida | Parámetros fuera de los tipos del contrato / Arguments outside the contract types | No representable: `i32` no admite nulo y el tipo de los enlaces es `Option`, no un valor / Not representable: `i32` admits no null and the link type is `Option`, not a value | La suite no ejerce ese caso / The suite does not exercise that case |

---

## ✅ Cobertura de pruebas / Test coverage

**ES:** Unidad y variante: una sola variante del contrato, la mutable de Rust
(`set_left(&mut self, …)`). Grupo lógico → archivo → runner: `TreeNode`,
`BinaryTree` y `Graph` viven en el mismo archivo de integración con cinco
ayudantes compartidos (`child`, `tree_with`, `graph_with`, `assert_tree_contains`
y `assert_counts`), y el runner es `cargo test`, que ejecuta el *crate* completo y
propaga los fallos. Cada fila es un caso del spec; las líneas apuntan a la
función de prueba.

**EN:** Unit and variant: a single contract variant, Rust's mutable one
(`set_left(&mut self, …)`). Logical group → file → runner: `TreeNode`,
`BinaryTree` and `Graph` live in the same integration file with five shared
helpers (`child`, `tree_with`, `graph_with`, `assert_tree_contains` and
`assert_counts`), and the runner is `cargo test`, which runs the whole crate and
propagates failures. Each row is a spec case; line numbers point at the test
function.

| Caso de la especificación / Specification case | Cubierto / Covered | Prueba / Test | Notas / Notes |
|---|---|:--:|---|
| **TreeNode:** Crear un nodo de árbol / Create a tree node | Sí | `tree_node_create` (:65) | `value()` = 10. |
| **TreeNode:** Enlaces iniciales ausentes / Absent initial links | Sí | `tree_node_absent_initial_links` (:75) | `left()` y `right()` en `None`. / `None`. |
| **TreeNode:** Enlazar dos hijos / Link two children | Sí | `tree_node_link_two_children` (:88) | `set_left(Some(Box::new(...)))` y `set_right`. |
| **TreeNode:** Recorrer cada enlace / Traverse each link | Sí | `tree_node_traverse_each_link` (:105) | Hijos y enlaces de los hijos. / Children and children's links. |
| **TreeNode:** Desenlazar hijos / Unlink children | Sí | `tree_node_unlink_children` (:134) | Izquierdo y luego derecho. / Left, then right. |
| **TreeNode:** Adaptación inmutable / Immutable adaptation | Omitido / Omitted | — | No aplica: `set_left` muta el receptor y devuelve `()`. / Not applicable: `set_left` mutates the receiver and returns `()`. |
| **BinaryTree:** Crear vacío / Create empty | Sí | `binary_tree_create_empty` (:172) | `is_empty`, `size`, `contains`. |
| **BinaryTree:** Insertar la raíz / Insert the root | Sí | `binary_tree_insert_root` (:186) | Tamaño 1 y pertenencia. / Size 1 and membership. |
| **BinaryTree:** Insertar menor y mayor / Insert lower and higher | Sí | `binary_tree_insert_lower_and_higher` (:204) | `50`, `30`, `70`. |
| **BinaryTree:** Ramas de varios niveles / Multilevel branches | Sí | `binary_tree_multilevel_branches` (:215) | `50`, `30`, `70`, `20`, `40`. |
| **BinaryTree:** Valor ausente en árbol no vacío / Missing value in nonempty tree | Sí | `binary_tree_missing_value_in_nonempty_tree` (:222) | `contains(99)` falla; tamaño 1. / `contains(99)` fails; size 1. |
| **BinaryTree:** Duplicado idempotente / Idempotent duplicate | Sí | `binary_tree_idempotent_duplicate` (:240) | Cuatro éxitos y tamaño 3. / Four successes and size 3. |
| **BinaryTree:** Inserciones ordenadas / Ordered insertions | Sí | `binary_tree_ordered_insertions` (:247) | `20`, `30`, `40`, `50`, `70`. |
| **Graph:** Crear vacío / Create empty | Sí | `graph_create_empty` (:270) | Contadores y consultas fallidas. / Counters and failed queries. |
| **Graph:** Vértice aislado / Isolated vertex | Sí | `graph_isolated_vertex` (:289) | `Some(vec![])`. |
| **Graph:** Etiquetas no consecutivas y arista dirigida / Nonconsecutive labels and directed edge | Sí | `graph_nonconsecutive_labels_and_directed_edge` (:308) | `has_edge(70, 10)` falla. / Fails. |
| **Graph:** Vértice duplicado conserva adyacencia / Duplicate vertex preserves adjacency | Sí | `graph_duplicate_vertex_preserves_adjacency` (:326) | Contadores intactos. / Counters intact. |
| **Graph:** Arista duplicada / Duplicate edge | Sí | `graph_duplicate_edge` (:345) | Ambas en éxito; una sola arista. / Both succeed; a single edge. |
| **Graph:** Extremo inexistente sin mutación / Missing endpoint without mutation | Sí | `graph_missing_endpoint_without_mutation` (:364) | Cuatro fallos; estado intacto. / Four failures; state intact. |
| **Graph:** Vecinos ordenados y contadores / Sorted neighbours and counts | Sí | `graph_sorted_neighbours_and_counts` (:404) | `[30,50,70]` y 4 aristas. / `[30,50,70]` and 4 edges. |
| **Graph:** Consulta de arista ausente / Missing-edge query | Sí | `graph_missing_edge_query` (:429) | 2 vértices, 0 aristas. / 2 vertices, 0 edges. |
| **Graph:** Vecinos de vértice inexistente / Neighbours of missing vertex | Sí | `graph_neighbours_of_missing_vertex` (:443) | `None` frente a `Some(vec![])`. / `None` versus `Some(vec![])`. |
| **Graph:** Bucle y duplicado / Self-loop and duplicate | Sí | `graph_self_loop_and_duplicate` (:459) | Vecinos `[10]`, 1 arista. / Neighbours `[10]`, 1 edge. |
| **Graph:** Aislamiento del resultado de vecinos / Neighbour-result isolation | Sí | `graph_neighbour_result_isolation` (:482) | Se muta la copia devuelta; el grafo y el resultado guardado no cambian. / The returned copy is mutated; the graph and the saved result do not change. |

**ES:** Totales: 23 casos ejecutados (24 del spec menos el condicional omitido)
en 1 archivo, con **71 macros de aserción** —66 dentro de los casos y 5 en los
ayudantes compartidos—. `cargo test` informa `23 passed; 0 failed`.

**EN:** Totals: 23 executed cases (24 in the spec minus the omitted conditional
one) in 1 file, with **71 assertion macros** —66 inside the cases and 5 in the
shared helpers—. `cargo test` reports `23 passed; 0 failed`.

---

## ⚠️ Limitaciones conocidas / Known limitations

| Limitación / Limitation | Impacto / Impact | Alternativa o plan / Workaround or plan |
|---|---|---|
| `BinaryTree::insert_at` es recursivo, no recursión de cola | Inserciones estrictamente ordenadas consumen pila `O(h)`; Rust no garantiza TCO | El descenso es `O(h)` por contrato; una versión iterativa con `&mut` encadenados exigiría `unsafe` o un bucle con `Option` reasignado, y el resultado observable sería el mismo. / Descent is `O(h)` by contract; an iterative version with chained `&mut` would need `unsafe` or a reassigned `Option` loop, with the same observable result. |
| Registro de vértices lineal y sin índice | `O(V)` por `add_vertex`, `has_edge` y `neighbors` | Es lo que fija la especificación: el registro se recorre y no se delega en un mapa. / This is what the specification fixes: the registry is traversed and not delegated to a map. |
| Inserción de vecinos `O(d(u))` por arista | Coste lineal en la adyacencia al añadir una arista | La adyacencia ordenada permite comparar vectores completos en las pruebas. / Sorted adjacency lets tests compare whole vectors. |
| `Box` por nodo en árbol y registro | Muchas asignaciones pequeñas y localidad de memoria menor que un `Vec` de nodos | El registro enlazado es la representación que exige el contrato para no usar índices de array. / The linked registry is the representation the contract requires so as not to use array indices. |
| `BinaryTree` y `Graph` derivan `Clone` y `PartialEq` | Una copia profunda del árbol o del grafo es `O(n)` | Las derivas son de la biblioteca estándar y no intervienen en las operaciones del contrato; existen para las comparaciones de la suite. / The derives come from the standard library and play no part in the contract operations; they exist for the suite's comparisons. |

---

## 📝 Notas de implementación / Implementation Notes

- **Recursión e iteración:** `BinaryTree::insert` recurre con
  `insert_at(&mut TreeNode, i32)` por el camino del ABB y devuelve si el valor era
  nuevo, de modo que el contador solo sube con altas reales; `contains` desciende
  con un bucle sobre `Option<&TreeNode>`. `Graph::find_vertex` y
  `find_vertex_mut` recorren la cadena de `next_vertex` con `while let`. Rust no
  garantiza TCO, así que la profundidad de la recursión de `insert_at` es la
  altura `h` del árbol.
- **Préstamos:** el descenso mutable usa `Option::as_deref_mut` en un `match`,
  porque el `while let ... { last = next }` sobre `&mut` deja el préstamo vivo y
  el compilador lo rechaza (`E0499`). `Graph::add_edge` valida los dos extremos
  con préstamos inmutables y solo después pide el vértice mutable (`E0502`).
  Anteponer el vértice nuevo con `self.vertices.take()` evita recorrer la cola
  solo para enlazar.
- **Indicadores:** la ausencia es `Option`; el éxito y el fallo de las altas y
  consultas son `true`/`false`. `Graph::neighbors` devuelve `Vec` clonado con
  `to_vec()`, así que mutar el resultado no toca el grafo ni el valor guardado.
- **Memoria:** sin `unsafe`, sin `Rc<RefCell<…>>` y sin arena; cada nodo se
  posee con `Box` y el registro y los enlaces son `Option<Box<…>>`.
- **Organización de pruebas:** un archivo de integración con cinco ayudantes que
  construyen instancias y comprueban contadores; `cargo test` ejecuta los 23
  casos y propaga el primer fallo de cada uno.
- **Compuertas:** `cargo fmt --check` y `cargo clippy --all-targets -- -D
  warnings` se ejecutaron antes de documentar; el módulo no tiene avisos.

**ES:** Este proyecto también está implementado en otros lenguajes. Explora el
repositorio principal para consultar las demás versiones.

**EN:** This project is also implemented in other languages. Explore the main
repository to see the other versions.

---

## 🔍 Checklist de validación / Validation checklist

- [x] La suite nativa se ejecutó y su salida real está copiada en este README.
- [x] Cada caso de la especificación tiene su fila en _Cobertura de pruebas_ (u `Omitido` con razón).
- [x] Cada desviación del pseudocódigo o de la ubicación esperada está en _Adaptaciones idiomáticas_.
- [x] Cada operación con fallo posible está en _Indicadores de fallo_.
- [x] No hay rutas absolutas del autor, credenciales ni salidas inventadas.
- [x] Los enlaces relativos resuelven dentro del repositorio y el documento es bilingüe.
- [x] Ninguna sección repite lo que ya dice la especificación.

---

## 📚 Referencias / References

| Tipo / Kind | Referencia / Reference |
|---|---|
| Especificación / Specification | [`07_Data_Structures_Advanced.md`](../../../../docs/core/algorithms/07_Data_Structures_Advanced.md) |
| Primera base del lenguaje / First language reference | `data_structures_basics` — [`README.md`](../data_structures_basics/README.md) |
| Segunda base del lenguaje / Second language reference | `numbers` — [`README.md`](../../foundations/numbers/README.md) |
| Plantilla de este README / This README's template | [`README_Template.md`](../../../../docs/README_Template.md) |
| Adaptaciones idiomáticas / Idiomatic adaptations | [`AGENT_Template.md`](../../../../docs/AGENT_Template.md) |
| Validación de la documentación / Documentation validation | [`WORKFLOW.md`](../../../../docs/WORKFLOW.md) |
| Documentación oficial del lenguaje / Language official docs | [The Rust Standard Library](https://doc.rust-lang.org/std/) |

---

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
