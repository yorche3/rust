# Naive Sort — Rust

Implementación de la especificación [05_Naive_Sort](https://yorche3.github.io/programming_languages/core/algorithms/05_Naive_Sort/) en **Rust**, usando la estructura Cargo de un proyecto tipo librería y el framework de pruebas integrado de Rust (`#[test]` + `cargo test`).

Los tres algoritmos elementales de ordenamiento $O(n^2)$ — **selection sort**, **bubble sort** e **insertion sort** — reciben el `Vec<i32>` por valor, reordenan su búfer *in-place* y devuelven el mismo vector, sin usar `sort`/`sort_by` ni ninguna otra ayuda de ordenamiento de la biblioteca estándar.

---

## 📂 Archivos y estructura / Files & Structure

| Archivo | Propósito |
|---------|-----------|
| [`Cargo.toml`](Cargo.toml) | Configuración del paquete Cargo y de la librería (`[lib] name = "naive_sort"`). |
| [`src/lib.rs`](src/lib.rs) | Punto de entrada de la librería; expone el módulo `naive_sort`. |
| [`src/naive_sort.rs`](src/naive_sort.rs) | Único módulo con las 3 funciones del contrato. |
| [`tests/naive_sort_tests.rs`](tests/naive_sort_tests.rs) | Suite de integración: 3 tests (uno por algoritmo) con los 7 casos de la tabla. |
| [`Cargo.lock`](Cargo.lock) | Versiones resueltas por Cargo (sin dependencias externas). |
| [`.gitignore`](.gitignore) | Ignora `target/`, los artefactos generados por Cargo. |

**Estructura de directorios / Directory structure:**

```text
naive_sort/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── src/
│   ├── lib.rs
│   └── naive_sort.rs          # Módulo NaiveSort: 3 algoritmos
├── tests/
│   └── naive_sort_tests.rs    # Suite: 3 tests × 7 casos
└── README.md
```

---

## 🛠️ Enfoque y construcción / Approach & Build

**ES:** Es un proyecto Cargo de tipo librería (`[lib]`), igual que `core/foundations/numbers/`. `src/lib.rs` expone el módulo `naive_sort` y la suite vive en `tests/`, así que consume la librería por su API pública como lo haría un usuario externo. No hay código de ejemplo ni `main`.

**EN:** This is a Cargo library project (`[lib]`), like `core/foundations/numbers/`. `src/lib.rs` exposes the `naive_sort` module and the suite lives in `tests/`, so it consumes the library through its public API as an external user would. There is no example code or `main`.

### Inicialización / Initialization

```bash
mkdir -p rust/core/algorithms/naive_sort
cd rust/core/algorithms/naive_sort
cargo init --lib
```

Después se añaden las funciones en `src/naive_sort.rs` y la suite en `tests/`.

---

## 📄 Configuración clave / Key Configuration

### `Cargo.toml` — paquete y librería

```toml
[package]
name = "naive_sort"
version = "0.1.0"
edition = "2024"

[lib]
name = "naive_sort"
path = "src/lib.rs"
```

### `src/naive_sort.rs` — contrato e implementación

**ES:** Las tres funciones tienen la misma forma: reciben el vector por valor (`mut arr: Vec<i32>`), lo reordenan en su propio búfer y lo devuelven. Con menos de dos elementos el vector vuelve sin cambios y ninguna función entra en pánico.

**EN:** All three functions share the same shape: they take the vector by value (`mut arr: Vec<i32>`), reorder it in its own buffer, and return it. With fewer than two elements the vector is returned unchanged and no function panics.

```rust
pub fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    if n < 2 {
        return arr;
    }
    for i in 0..n-1 {
        let mut min_index = i;
        for j in i+1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
    arr
}
```

| Elemento del contrato | Representación en Rust |
| --------------------- | ---------------------- |
| Array de enteros | `Vec<i32>` (índices 0-based) |
| Caso nulo | Omitido: `Vec<i32>` no admite `null` y la fase todavía no usa `Option`/`Result` como indicador de fallo |
| Array vacío | `Vec::new()` (`len() == 0`) |
| Orden | in-place sobre el búfer del `Vec`, devuelve el mismo vector |
| Nombres de la especificación | `selection_sort`, `bubble_sort`, `insertion_sort` (snake_case, idénticos a la especificación y a la convención de Rust) |

### Suite de pruebas / Test suite

**ES:** Las constantes son arrays `[i32; N]`, la tabla `CASES` reúne descripción, entrada y salida esperada, y un helper compartido recibe el nombre del algoritmo y la función a probar. Cada caso ordena una copia (`to_vec()`) del fixture, porque el orden es *in-place*.

**EN:** The constants are `[i32; N]` arrays, the `CASES` table holds description, input, and expected output, and a shared helper receives the algorithm name and the function under test. Each case sorts a `to_vec()` copy of the fixture, because sorting is *in-place*.

```rust
// Helper compartido: recibe el nombre del algoritmo y la función a probar, y
// ejecuta todos los casos con el mensaje descriptivo del contrato.
fn assert_sorts_all_cases(algorithm: &str, sort: fn(Vec<i32>) -> Vec<i32>) {
    for test_case in &CASES {
        assert_eq!(
            sort(test_case.input.to_vec()),
            test_case.expected,
            "{} should sort {}",
            algorithm,
            test_case.description
        );
    }
}
```

---

## 🚀 Compilación y ejecución / Build & Run

### Requisitos / Requirements

- **Rust toolchain** (en este entorno, 1.98.1) con `cargo` disponible; puede requerir `source "$HOME/.cargo/env"` en una sesión nueva.

```bash
rustc --version
cargo --version
```

### Verificación estática y pruebas / Static check & tests

```bash
cd rust/core/algorithms/naive_sort
cargo clippy --all-targets -- -D warnings
cargo test
```

**Salida real / Actual output:**

```text
$ cargo clippy --all-targets -- -D warnings
    Checking naive_sort v0.1.0 (/home/yorche3/programming_languages/rust/core/algorithms/naive_sort)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
```

```text
$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/naive_sort-63afbfd35a0eecc8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/naive_sort_tests.rs (target/debug/deps/naive_sort_tests-d7ece2f332dec956)

running 3 tests
test bubble_sort ... ok
test insertion_sort ... ok
test selection_sort ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests naive_sort

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

> **ES:** Clippy con `-D warnings` es la verificación estática y no reporta nada; `cargo test` compila la librería y la suite de integración y ejecuta 3 tests, uno por algoritmo, cada uno con los 7 casos de la tabla (21 aserciones).
> **EN:** Clippy with `-D warnings` is the static check and reports nothing; `cargo test` compiles the library and the integration suite and runs 3 tests, one per algorithm, each with the 7 cases of the table (21 assertions).

---

## 🧠 Algoritmos y operaciones / Algorithms & Operations

| Función / Algorithm | Enfoque / Approach | Descripción / Description |
| ------------------- | ------------------ | ------------------------- |
| `selection_sort(arr)` | iterativo, in-place | Busca el mínimo del tramo no ordenado con `min_index` y lo intercambia al inicio con `arr.swap`. $O(n^2)$ siempre. |
| `bubble_sort(arr)` | iterativo, in-place, con bandera | Compara adyacentes e intercambia; sale antes con `if !swapped { break; }` cuando no hubo intercambios. $O(n^2)$ peor/promedio, $O(n)$ mejor. |
| `insertion_sort(arr)` | iterativo, in-place | Guarda `key`, desplaza el sub-array ordenado con `while j > 0 && arr[j-1] > key` y lo inserta en su posición. $O(n^2)$ peor/promedio, $O(n)$ mejor. |

| Caso (descripción en la suite) | Entrada | Salida esperada |
| ------------------------------ | ------- | --------------- |
| an unsorted array | `[5, 2, 9, 1, 5, 6]` | `[1, 2, 5, 5, 6, 9]` |
| an already sorted array | `[1, 2, 3, 4, 5]` | `[1, 2, 3, 4, 5]` |
| a reverse ordered array | `[5, 4, 3, 2, 1]` | `[1, 2, 3, 4, 5]` |
| an array of identical elements | `[7, 7, 7, 7]` | `[7, 7, 7, 7]` |
| an array with negative numbers | `[3, -1, 4, -5, 0]` | `[-5, -1, 0, 3, 4]` |
| a single element array | `[42]` | `[42]` |
| an empty array | `[]` | `[]` |

---

## 📝 Notas de implementación / Implementation Notes

- **ES:** Divergencia idiomática aceptada: las funciones reciben el `Vec` **por valor** y devuelven el mismo vector reordenado en su propio búfer, variante que la especificación permite («in-place o retornando una copia ordenada»). En Rust esto evita los préstamos mutables en la firma y deja el orden *in-place* dentro de la función.
- **EN:** Accepted idiomatic divergence: the functions take the `Vec` **by value** and return the same vector reordered in its own buffer, a variant the specification allows ("in-place or returning a sorted copy"). In Rust this avoids mutable borrows in the signature while keeping sorting *in-place* inside the function.
- **ES:** Caso nulo omitido: `Vec<i32>` no admite `null` y esta fase todavía no usa `Option`/`Result` como indicador de fallo, así que una entrada nula no es representable en la firma y no hay indicador que comprobar. Se conservan los 7 casos de la especificación (mismo criterio que OCaml). Ninguna función entra en pánico.
- **EN:** Null case omitted: `Vec<i32>` does not admit `null` and this phase does not yet use `Option`/`Result` as the failure indicator, so a null input is not representable in the signature and there is no indicator to check. The 7 cases of the specification are kept (same criterion as OCaml). No function panics.
- **ES:** `bubble_sort` conserva la optimización de salida temprana: la bandera `swapped` y `if !swapped { break; }` reproducen el `if not swapped: break` del pseudocódigo (mejor caso $O(n)$). La bandera no es observable en la salida, así que su presencia se verifica contra el pseudocódigo.
- **EN:** `bubble_sort` keeps the early-exit optimization: the `swapped` flag and `if !swapped { break; }` reproduce the pseudocode's `if not swapped: break` (best case $O(n)$). The flag is not observable in the output, so its presence is verified against the pseudocode.
- **ES:** Los bucles del pseudocódigo se traducen a rangos exclusivos de Rust: `0..n-1` y `i+1..n` en `selection_sort`, y `0..n-i-1` en el bucle interior de `bubble_sort` (el equivalente exacto de `for j = 0 to n - 2 - i` con índices 0-based).
- **EN:** The pseudocode loops translate to Rust's exclusive ranges: `0..n-1` and `i+1..n` in `selection_sort`, and `0..n-i-1` in the inner loop of `bubble_sort` (the exact equivalent of `for j = 0 to n - 2 - i` with 0-based indexes).
- **ES:** `insertion_sort` traduce el `while j >= 0 and arr[j] > key` del pseudocódigo a `while j > 0 && arr[j-1] > key` sobre un `usize`, desplazando con `arr[j] = arr[j-1]` e insertando en `arr[j]`; así se evita el índice negativo y el cast a `isize`, con el mismo comportamiento (la comparación estricta lo mantiene estable).
- **EN:** `insertion_sort` translates the pseudocode's `while j >= 0 and arr[j] > key` into `while j > 0 && arr[j-1] > key` over a `usize`, shifting with `arr[j] = arr[j-1]` and inserting at `arr[j]`; this avoids negative indexes and the `isize` cast, with the same behaviour (the strict comparison keeps it stable).
- **ES:** El único intercambio es `arr.swap(i, j)` de `Vec`, que intercambia dos posiciones paso a paso; no se invoca `sort`, `sort_by`, `sort_unstable` ni ninguna otra ayuda de ordenamiento o selección de la biblioteca estándar.
- **EN:** The only swap is `Vec::swap(i, j)`, which exchanges two positions step by step; `sort`, `sort_by`, `sort_unstable`, or any other standard-library sorting or selection helper is never called.
- **ES:** Nota de desviación respecto a la ubicación esperada: se conserva `src/naive_sort.rs` (solo cambia la extensión) y la suite se llama `tests/naive_sort_tests.rs`, el layout de integración de Cargo que ya usa `numbers/`; no se añade `run_tests.rs` porque `cargo test` descubre `tests/` por sí solo. El caso nulo no representable se omite en vez de introducir `Option`.
- **EN:** Deviation note from the expected location: `src/naive_sort.rs` is kept (only the extension changes) and the suite is named `tests/naive_sort_tests.rs`, the Cargo integration layout already used by `numbers/`; no `run_tests.rs` is added because `cargo test` discovers `tests/` on its own. The unrepresentable null case is omitted instead of introducing `Option`.

---

## 🌐 Otras implementaciones / Other implementations

Este proyecto también está implementado en otros lenguajes. Explora el [repositorio principal](https://github.com/yorche3/programming_languages) para ver todas las versiones.

---

*[← Volver a Algoritmos Puros](../README.md)*

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
