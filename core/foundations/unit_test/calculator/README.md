# Calculator — Rust

Implementación de la especificación [03_Unit_Test_Calculator](https://yorche3.github.io/programming_languages/core/foundations/03_Unit_Test_Calculator/) en **Rust**, usando la configuración Cargo de un proyecto tipo librería y el framework de pruebas integrado de Rust (`#[test]` + `cargo test`).

No se requiere instalar una biblioteca externa de testing: el runner y las aserciones forman parte del toolchain estándar de Rust.

---

## 📂 Archivos y estructura / Files & Structure

| Archivo | Propósito |
|---------|-----------|
| [`Cargo.toml`](Cargo.toml) | Configuración del paquete Cargo y de la librería. |
| [`src/lib.rs`](src/lib.rs) | Punto de entrada de la librería; expone el módulo `calculator`. |
| [`src/calculator.rs`](src/calculator.rs) | Implementación de las 5 operaciones educativas. |
| [`tests/calculator_test.rs`](tests/calculator_test.rs) | Tests de integración: 5 funciones `#[test]`, una por operación. |
| [`.gitignore`](.gitignore) | Ignora `target/`, los artefactos generados por Cargo. |

```text
calculator/
├── Cargo.toml
├── .gitignore
├── src/
│   ├── lib.rs
│   └── calculator.rs
├── tests/
│   └── calculator_test.rs
└── README.md
```

---

## 🛠️ Enfoque y construcción / Approach & Build

**ES:** Es un proyecto Cargo de tipo librería (`[lib]`). `src/lib.rs` expone `calculator`, y `tests/calculator_test.rs` importa la librería como lo haría un consumidor externo. Cada operación sigue las implementaciones educativas de la especificación:

- `addition`: suma directa.
- `subtraction`: resta directa.
- `multiplication`: suma repetitiva, sin usar `*`.
- `division`: resta repetitiva, sin usar `/`.
- `modulus`: reutiliza `division`, `multiplication` y `subtraction`, sin usar `%`.

**EN:** This is a Cargo library project (`[lib]`). `src/lib.rs` exposes `calculator`, and `tests/calculator_test.rs` imports the library as an external consumer would. Each operation follows the specification's educational implementation:
 
 - `addition`: direct addition.
 - `subtraction`: direct subtraction.
 - `multiplication`: repeated addition, without `*`.
 - `division`: repeated subtraction, without `/`.
 - `modulus`: reuses `division`, `multiplication`, and `subtraction`, without `%`.

### Inicialización / Initialization

```bash
mkdir -p rust/core/foundations/unit_test/calculator/{src,tests}
cd rust/core/foundations/unit_test/calculator
cargo init --lib
```

Después se añaden las funciones en `src/calculator.rs` y los tests en `tests/calculator_test.rs`.

---

## 📄 Archivos de configuración clave / Key Configuration Files

### `Cargo.toml`

```toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2024"

[lib]
name = "calculator"
path = "src/lib.rs"
```

| Campo | Propósito |
|-------|-----------|
| `[package]` | Metadatos del paquete Cargo. |
| `edition = "2024"` | Usa la edición moderna de Rust disponible en el toolchain. |
| `[lib]` | Declara explícitamente que el proyecto produce una librería. |
| `path = "src/lib.rs"` | Define el punto de entrada de la librería. |

### `src/lib.rs`

```rust
pub mod calculator;
```

`pub mod calculator` hace accesibles las operaciones como `calculator::addition(...)` desde los tests de integración.

### `tests/calculator_test.rs`

**ES:** El framework integrado de Rust descubre las funciones marcadas con `#[test]`. Cargo ejecuta todos los tests y reporta cualquier fallo.

**EN:** Rust's built-in framework discovers functions marked with `#[test]`. Cargo runs every test and reports any failure.

```rust
use calculator::calculator;

#[test]
fn addition() {
    assert_eq!(calculator::addition(2, 3), 5);
}
```

---

## 🚀 Compilación y ejecución / Build & Run

### Requisitos / Requirements

- **Rust toolchain** (`rustc` y `cargo`), instalado mediante [rustup](https://rustup.rs/).
- Toolchain verificado en este entorno: `rustc 1.98.1`, `cargo 1.98.1`.

```bash
source "$HOME/.cargo/env"
rustc --version
cargo --version
```

### Ejecutar las pruebas / Run tests

```bash
cd rust/core/foundations/unit_test/calculator
cargo test
```

### Salida esperada / Expected output

```text
running 5 tests
test addition ... ok
test division ... ok
test modulus ... ok
test multiplication ... ok
test subtraction ... ok

test result: ok. 5 passed; 0 failed
```

> **ES:** El formato exacto puede variar entre versiones de Cargo, pero deben pasar los 5 tests y no debe haber fallos.
> **EN:** The exact format may vary between Cargo versions, but all 5 tests must pass with no failures.

---

## 📝 Notas de implementación / Implementation Notes

- **ES:** Los tests están en `tests/`, por lo que son tests de integración y consumen la librería a través de su API pública.
- **EN:** Tests live in `tests/`, so they are integration tests and consume the library through its public API.
- **ES:** La división por cero no se maneja en este ejercicio básico, siguiendo el pseudocódigo de la especificación; los casos probados usan valores válidos.
- **EN:** Division by zero is not handled in this basic exercise, following the specification's pseudocode; tested cases use valid values.
- **ES:** El proyecto usa únicamente el toolchain estándar de Rust; no se añade `dev-dependency` externa para testing.
- **EN:** The project uses only Rust's standard toolchain; no external testing `dev-dependency` is added.

---

## 🌐 Otras implementaciones / Other implementations

Este proyecto también está implementado en otros lenguajes. Explora el [repositorio principal](https://github.com/yorche3/programming_languages) para ver todas las versiones.

---

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
