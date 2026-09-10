# Rust

Proyectos en **Rust**, con programas independientes compilados mediante `rustc` y proyectos tipo librería gestionados con **Cargo** y probados mediante el framework integrado de Rust (`#[test]` + `cargo test`).

---

## 📂 Módulos / Modules

| Módulo | Descripción |
| ------ | ----------- |
| [`core/foundations/`](core/foundations/) | **Fase 0 — Fundamentos**: `helloworld`, `hellouser`, `unit_test/calculator`, `numbers` |

---

## ▶️ Comenzar / Getting Started

```bash
# Cargar el toolchain instalado con rustup
source "$HOME/.cargo/env"

# Hello, World!
cd core/foundations/helloworld
rustc helloworld.rs -o helloworld
./helloworld

# Hello, User!
cd ../hellouser
rustc hellouser.rs -o hellouser
printf 'Ada\n' | ./hellouser

# Calculator tests
cd ../unit_test/calculator
cargo test

# Numbers tests
cd ../../numbers
cargo test
```

---

## 📦 Requisitos / Requirements

| Herramienta | Uso | Instalación/verificación |
| ----------- | --- | ------------------------ |
| [Rust toolchain](https://www.rust-lang.org/tools/install) | `rustc`, `cargo`, `rustfmt` | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| [rustup](https://rustup.rs/) | Instalación y gestión del toolchain | `rustup --version` |

```bash
# Cargar Rust en la sesión actual
source "$HOME/.cargo/env"

# Verificar la instalación
rustc --version
cargo --version
rustfmt --version
```

> **ES:** En este entorno, `$HOME/.cargo/env` se carga desde los perfiles del shell (`~/.bash_profile`, `~/.profile` y `~/.bashrc`).
> **EN:** In this environment, `$HOME/.cargo/env` is loaded from the shell profiles (`~/.bash_profile`, `~/.profile`, and `~/.bashrc`).

No se requieren dependencias externas para testing: `cargo test` usa el framework integrado de Rust.

---

## 🏗️ Tipos de proyecto / Project Types

### 1. Programa independiente (`rustc`)

**ES:** Un único archivo `.rs` con una función `main`, compilado directamente con `rustc` y ejecutado como binario nativo. Es el formato usado por `helloworld` y `hellouser`.

**EN:** A single `.rs` file with a `main` function, compiled directly with `rustc` and run as a native binary. This is the format used by `helloworld` and `hellouser`.

```bash
rustc <File>.rs -o <binary>
./<binary>
```

### 2. Proyecto Cargo tipo librería

**ES:** `calculator` y `numbers` usan `Cargo.toml`, `src/lib.rs`, módulos en `src/` y tests de integración en `tests/`. Cargo compila la librería, descubre todas las funciones `#[test]` y ejecuta las suites con un solo comando.

**EN:** `calculator` and `numbers` use `Cargo.toml`, `src/lib.rs`, modules under `src/`, and integration tests under `tests/`. Cargo builds the library, discovers all `#[test]` functions, and runs the suites with one command.

```bash
cargo test
cargo fmt --check
```

---

## 🔁 Decisión de TCO / TCO Decision

Rust tiene iteración nativa, pero no garantiza Tail Call Optimization en su modelo estándar. Por eso `numbers` conserva `_acc` como puente educativo sin suite propia y prueba `_rec` e `_ite`: **TCO ❌ + iteración ✅ → 2 suites, 10 tests agrupados por función y 22 casos/assertions**.

Rust has native iteration but does not guarantee Tail Call Optimization in its standard compilation model. Therefore, `numbers` keeps `_acc` as an educational bridge without a dedicated suite and tests `_rec` and `_ite`: **TCO ❌ + iteration ✅ → 2 suites, 10 tests grouped by function, and 22 cases/assertions**.

---

## 🧪 Convenciones de pruebas / Testing Conventions

**ES:** Cada proyecto Cargo agrupa los tests por función: un `#[test]` por algoritmo con varios `assert_eq!` dentro. `cargo test` ejecuta calculator y numbers sin instalar frameworks externos.

**EN:** Each Cargo project groups tests by function: one `#[test]` per algorithm with multiple `assert_eq!` calls inside. `cargo test` runs calculator and numbers without installing external frameworks.

---

## 🌐 Otras implementaciones / Other implementations

Este proyecto también está implementado en otros lenguajes. Explora el [repositorio principal](https://github.com/yorche3/programming_languages) para ver todas las versiones.

---

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*