# 🚀 Fundamentos / Foundations — Rust

Implementación de los ejercicios de la sección [Fundamentos / Foundations](https://yorche3.github.io/programming_languages/core/foundations/) del repositorio principal en **Rust**.

---

## 📖 Descripción / Description

**ES:** Esta sección reúne los conceptos esenciales para empezar a trabajar con **Rust**. Cubre desde programas independientes compilados directamente con `rustc` hasta proyectos Cargo de tipo librería con tests de integración usando el framework integrado de Rust.

**EN:** This section brings together the essential concepts for getting started with **Rust**. It covers standalone programs compiled directly with `rustc` and Cargo library projects with integration tests using Rust's built-in testing framework.

---

## 📁 Estructura / Structure

```text
rust/
└── core/
    └── foundations/
        ├── README.md              # Este archivo / This file
        ├── helloworld/            # 01_Hello_World — Primer programa
        │   ├── helloworld.rs
        │   └── README.md
        ├── hellouser/             # 02_Hello_User — Entrada y salida
        │   ├── hellouser.rs
        │   └── README.md
        ├── unit_test/
        │   └── calculator/        # 03_Unit_Test_Calculator — Pruebas unitarias
        │       ├── Cargo.toml
        │       ├── Cargo.lock
        │       ├── src/
        │       │   ├── lib.rs
        │       │   └── calculator.rs
        │       ├── tests/
        │       │   └── calculator_test.rs
        │       ├── .gitignore
        │       └── README.md
        └── numbers/               # 04_Numbers — Algoritmos numéricos
            ├── Cargo.toml
            ├── Cargo.lock
            ├── src/
            │   ├── lib.rs
            │   └── numbers.rs
            ├── tests/
            │   ├── recursive_tests.rs
            │   └── iterative_tests.rs
            ├── .gitignore
            └── README.md
```

---

## 🔢 Progresión / Progression

| Especificación | Proyecto | Conceptos | Tests | Dependencias |
| -------------- | -------- | --------- | :---: | ------------ |
| [`01_Hello_World`](https://yorche3.github.io/programming_languages/core/foundations/01_Hello_World/) | [`helloworld/`](helloworld/) | `fn main`, `println!`, compilación con `rustc` | — | Solo Rust |
| [`02_Hello_User`](https://yorche3.github.io/programming_languages/core/foundations/02_Hello_User/) | [`hellouser/`](hellouser/) | `std::io`, `String`, `read_line`, formato | — | Solo Rust |
| [`03_Unit_Test_Calculator`](https://yorche3.github.io/programming_languages/core/foundations/03_Unit_Test_Calculator/) | [`unit_test/calculator/`](unit_test/calculator/) | Cargo lib, módulos públicos, `#[test]`, `assert_eq!` | 5 | Toolchain estándar |
| [`04_Numbers`](https://yorche3.github.io/programming_languages/core/foundations/04_Numbers/) | [`numbers/`](numbers/) | Recursión, acumuladores, iteración y TCO | 10 (22 casos) | Toolchain estándar |

---

## 🛠️ Enfoque general / General Approach

**ES:** Los proyectos de esta sección siguen una progresión gradual:

1. **Hello World** y **Hello User**: programas independientes en archivos `.rs`, compilados directamente con `rustc`. No usan Cargo ni dependencias externas.
2. **Calculator**: primer proyecto con estructura Cargo tipo librería (`Cargo.toml`, `src/lib.rs`, módulo en `src/` y tests de integración en `tests/`). Usa el framework integrado de Rust con `#[test]` y `assert_eq!`.
3. **Numbers**: librería Cargo con 15 funciones en tres enfoques. Rust tiene iteración nativa, pero no garantiza Tail Call Optimization; por eso `_acc` permanece como puente educativo sin suite propia. Se prueban `_rec` e `_ite`: **TCO ❌ + iteración ✅ → 2 suites, 10 tests agrupados por función y 22 casos/assertions**.

**EN:** The projects in this section follow a gradual progression:

1. **Hello World** and **Hello User**: standalone `.rs` programs compiled directly with `rustc`. They use neither Cargo nor external dependencies.
2. **Calculator**: the first Cargo library project (`Cargo.toml`, `src/lib.rs`, a module under `src/`, and integration tests under `tests/`). It uses Rust's built-in `#[test]` and `assert_eq!` framework.
3. **Numbers**: a Cargo library with 15 functions in three approaches. Rust has native iteration but does not guarantee Tail Call Optimization; therefore `_acc` remains an educational bridge without its own suite. `_rec` and `_ite` are tested: **TCO ❌ + iteration ✅ → 2 suites, 10 tests grouped by function, and 22 cases/assertions**.

---

## 📦 Requisitos / Requirements

| Herramienta | Uso | Verificación |
| ----------- | --- | ------------ |
| [Rust toolchain](https://www.rust-lang.org/tools/install) | `rustc`, `cargo`, `rustfmt` | `rustc --version` |
| [rustup](https://rustup.rs/) | Instalación y gestión del toolchain | `rustup --version` |

En este entorno, Rust se carga desde `$HOME/.cargo/env` mediante los perfiles del shell:

```bash
source "$HOME/.cargo/env"
rustc --version
cargo --version
```

No se requiere instalar un framework de testing externo: `cargo test` usa las herramientas integradas de Rust.

---

## 🚀 Ejecución rápida / Quick Start

### Hello World

```bash
cd rust/core/foundations/helloworld
rustc helloworld.rs -o helloworld
./helloworld
```

### Hello User

```bash
cd rust/core/foundations/hellouser
rustc hellouser.rs -o hellouser
printf 'Ada\n' | ./hellouser
```

### Calculator (Cargo tests)

```bash
cd rust/core/foundations/unit_test/calculator
cargo test
```

Salida resumida esperada:

```text
test result: ok. 5 passed; 0 failed
```

### Numbers (Cargo tests)

```bash
cd rust/core/foundations/numbers
cargo test
```

Salida resumida esperada:

```text
test result: ok. 5 passed; 0 failed
test result: ok. 5 passed; 0 failed
```

---

## 🧪 Convenciones de pruebas / Testing Conventions

**ES:** Los proyectos Cargo usan tests de integración en `tests/`. Cada suite agrupa los casos por función: un `#[test]` por algoritmo con varios `assert_eq!` dentro. `cargo test` descubre y ejecuta todas las suites con un solo comando.

**EN:** Cargo projects use integration tests under `tests/`. Each suite groups cases by function: one `#[test]` per algorithm with multiple `assert_eq!` calls inside. `cargo test` discovers and runs every suite with one command.

---

## 🧹 Artefactos de compilación / Build Artefacts

Cargo crea `target/` durante la compilación y testing. Cada proyecto Cargo incluye un `.gitignore` local para excluirlo; `Cargo.lock` se conserva para registrar el estado reproducible de las dependencias del proyecto.

Cargo creates `target/` during compilation and testing. Each Cargo project includes a local `.gitignore` to exclude it; `Cargo.lock` is kept to record the reproducible dependency state of the project.

---

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
