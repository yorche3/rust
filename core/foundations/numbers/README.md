# Numbers — Rust

Implementación de la especificación [04_Numbers](https://yorche3.github.io/programming_languages/core/foundations/04_Numbers/) en **Rust**, usando la estructura Cargo de un proyecto tipo librería y el framework de pruebas integrado de Rust (`#[test]` + `cargo test`).

No se requiere instalar una biblioteca externa de testing.

---

## 📂 Archivos y estructura / Files & Structure

| Archivo | Propósito |
|---------|-----------|
| [`Cargo.toml`](Cargo.toml) | Configuración del paquete Cargo y de la librería. |
| [`src/lib.rs`](src/lib.rs) | Punto de entrada de la librería; expone el módulo `numbers`. |
| [`src/numbers.rs`](src/numbers.rs) | Único módulo con las 15 funciones (3 enfoques × 5 algoritmos) y 4 helpers privados. |
| [`tests/recursive_tests.rs`](tests/recursive_tests.rs) | Suite `_rec`: 5 tests agrupados por función, con 11 casos. |
| [`tests/iterative_tests.rs`](tests/iterative_tests.rs) | Suite `_ite`: 5 tests agrupados por función, con 11 casos. |
| [`.gitignore`](.gitignore) | Ignora `target/`, los artefactos generados por Cargo. |

```text
numbers/
├── Cargo.toml
├── .gitignore
├── src/
│   ├── lib.rs
│   └── numbers.rs
├── tests/
│   ├── recursive_tests.rs
│   └── iterative_tests.rs
└── README.md
```

---

## 🛠️ Enfoque y construcción / Approach & Build

**ES:** Es un proyecto Cargo de tipo librería (`[lib]`). `src/lib.rs` expone el módulo `numbers`, y las suites de `tests/` consumen la librería mediante su API pública, como lo haría un usuario externo.

Los cinco algoritmos se implementan en tres enfoques:

| Enfoque | Sufijo | Ejemplo | ¿Tiene suite propia? |
|---------|--------|---------|:--------------------:|
| Recursivo directo | `_rec` | `fibonacci_rec` | ✅ Sí |
| Recursivo con acumulador | `_acc` | `fibonacci_acc` | ❌ No (ver TCO) |
| Iterativo | `_ite` | `fibonacci_ite` | ✅ Sí |

**Combinación aplicada:** TCO ❌ + iteración ✅ → `_rec` + `_ite` = **2 suites, 10 tests agrupados por función y 22 casos/assertions**.

**EN:** This is a Cargo library project (`[lib]`). `src/lib.rs` exposes the `numbers` module, and the suites in `tests/` consume the library through its public API, as an external user would.

The five algorithms are implemented through three approaches:

| Approach | Suffix | Example | Dedicated suite? |
|----------|--------|---------|:----------------:|
| Direct recursion | `_rec` | `fibonacci_rec` | ✅ Yes |
| Accumulator recursion | `_acc` | `fibonacci_acc` | ❌ No (see TCO) |
| Iterative | `_ite` | `fibonacci_ite` | ✅ Yes |

**Applied combination:** No guaranteed TCO + native iteration ✅ → `_rec` + `_ite` = **2 suites, 10 tests grouped by function, and 22 cases/assertions**.

### Inicialización / Initialization

```bash
mkdir -p rust/core/foundations/numbers/{src,tests}
cd rust/core/foundations/numbers
cargo init --lib
```

Después se añaden las funciones en `src/numbers.rs` y las suites en `tests/`.

---

## 📄 Archivos de configuración clave / Key Configuration Files

### `Cargo.toml`

```toml
[package]
name = "numbers"
version = "0.1.0"
edition = "2024"

[lib]
name = "numbers"
path = "src/lib.rs"
```

### `src/lib.rs`

```rust
pub mod numbers;
```

### `src/numbers.rs`

**ES:** El módulo contiene los 15 métodos. Los helpers `_help` no son `pub`, por lo que permanecen privados al módulo. El código solo contiene comentarios de sección y no documentación inline extensa.

**EN:** The module contains all 15 functions. `_help` helpers are not `pub`, so they remain private to the module. The source only contains section comments and no extensive inline documentation.

```rust
// Direct recursion (_rec)

pub fn fibonacci_rec(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
}

// Accumulator recursion (_acc): educational bridge

pub fn fibonacci_acc(n: i32) -> i32 {
    fibonacci_acc_help(n, 0, 1)
}

fn fibonacci_acc_help(n: i32, acc2: i32, acc1: i32) -> i32 {
    if n <= 0 {
        return acc2;
    }
    if n <= 2 {
        return acc1 + acc2;
    }
    fibonacci_acc_help(n - 1, acc1, acc1 + acc2)
}

// Iterative (_ite)

pub fn fibonacci_ite(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut acc2 = 0;
    let mut acc1 = 1;
    for _ in 2..=n {
        let temp = acc1 + acc2;
        acc2 = acc1;
        acc1 = temp;
    }
    acc1
}
```

| Algoritmo | `_rec` | `_acc` | `_ite` |
|-----------|--------|--------|--------|
| `sum_of_first_n` | `n + sum_rec(n - 1)` | helper con `n + acc` | `for 1..=n` |
| `factorial` | `n * factorial_rec(n - 1)` | helper con `n * acc` | `for 2..=n` |
| `fibonacci` | suma de dos llamadas | helper con `acc2`, `acc1` | intercambio con `temp` |
| `greatest_common_divisor` | Euclides recursivo | helper de Euclides | `while b != 0` |
| `least_common_multiple` | `(a * b) / gcd` | `(a * b) / gcd` | `(a * b) / gcd` |

### Suites de pruebas — framework integrado de Rust

**ES:** Hay dos suites de integración. Cada suite agrupa los casos por función: un `#[test]` por algoritmo, con varios `assert_eq!` dentro. Esto produce 5 tests de Rust por suite y 22 casos/assertions en total.

**EN:** There are two integration suites. Each suite groups cases by function: one `#[test]` per algorithm, with multiple `assert_eq!` calls inside. This produces 5 Rust tests per suite and 22 total cases/assertions.

```rust
use numbers::numbers;

#[test]
fn fibonacci_rec() {
    assert_eq!(numbers::fibonacci_rec(0), 0);
    assert_eq!(numbers::fibonacci_rec(1), 1);
    assert_eq!(numbers::fibonacci_rec(6), 8);
}
```

---

## 🚀 Compilación y ejecución / Build & Run

### Requisitos / Requirements

- **Rust toolchain**, normalmente instalado mediante [rustup](https://rustup.rs/).
- `rustc` y `cargo`.
- Toolchain utilizado en esta implementación: `rustc 1.98.1`, `cargo 1.98.1`.

```bash
source "$HOME/.cargo/env"
rustc --version
cargo --version
```

No se requiere instalar crates externos para las pruebas.

### Ejecutar todas las pruebas / Run all tests

```bash
cd rust/core/foundations/numbers
cargo test
```

### Salida esperada / Expected output

```text
running 5 tests
...
test result: ok. 5 passed; 0 failed

running 5 tests
...
test result: ok. 5 passed; 0 failed
```

> **ES:** Cargo también ejecuta los targets de la librería y los doc-tests, que pueden mostrar `0 tests`; las dos suites de integración deben mostrar 5 tests cada una.
> **EN:** Cargo also runs the library and doc-test targets, which may show `0 tests`; both integration suites should report 5 tests each.

### Formato / Formatting

```bash
cargo fmt --check
```

---

## 🔁 Sobre recursión con acumulador y Tail Call Optimization (TCO)

**ES:**
La recursión con acumulador coloca el estado de la operación en parámetros y deja la llamada recursiva como última operación del helper. Sin embargo, Rust no garantiza Tail Call Optimization en su modelo de compilación estándar. Por eso `_acc` se conserva como puente didáctico entre `_rec` e `_ite`, pero no tiene una suite dedicada.

La combinación aplicada es **TCO ❌ + iteración ✅**: se prueban `_rec` e `_ite`, mientras `_acc` permanece disponible en el módulo sin formar parte del conteo de tests.

**EN:**
Accumulator recursion stores the operation state in parameters and leaves the recursive call as the helper's final operation. However, Rust does not guarantee Tail Call Optimization in its standard compilation model. Therefore, `_acc` is kept as an educational bridge between `_rec` and `_ite`, but it has no dedicated suite.

The applied combination is **No guaranteed TCO + native iteration ✅**: `_rec` and `_ite` are tested, while `_acc` remains available in the module but is excluded from the test count.

---

## 📝 Notas de implementación / Implementation Notes

- **ES:** Los tests están en `tests/`, por lo que Cargo los trata como tests de integración y el módulo debe exponer públicamente las funciones probadas.
- **EN:** Tests live in `tests/`, so Cargo treats them as integration tests and the module must publicly expose the tested functions.
- **ES:** La división por cero no se maneja en este ejercicio básico; los casos definidos por la especificación usan divisores válidos.
- **EN:** Division by zero is not handled in this basic exercise; the specification's cases use valid divisors.
- **ES:** `for 1..=0` y `for 2..=0` no ejecutan iteraciones para estos enteros, por lo que los valores base de suma y factorial se conservan.
- **EN:** `for 1..=0` and `for 2..=0` perform no iterations for these integers, preserving the base values for sum and factorial.
- **ES:** El MCM usa división entera `/` sobre `i32`; los casos de la especificación producen resultados exactos.
- **EN:** LCM uses integer `/` over `i32`; the specification's cases produce exact results.
- **ES:** El operador `%` se usa en MCD; la restricción de no usarlo pertenece al módulo `calculator` de la especificación 03, no a `numbers`.
- **EN:** `%` is used in GCD; the restriction against it belongs to specification 03's `calculator` module, not `numbers`.

---

## 🌐 Otras implementaciones / Other implementations

Este proyecto también está implementado en otros lenguajes. Explora el [repositorio principal](https://github.com/yorche3/programming_languages) para ver todas las versiones.

---

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
