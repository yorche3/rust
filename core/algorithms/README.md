# 🚀 Algoritmos Puros / Algorithms Pure — Rust

Implementaciones de la [Fase 1 — Algoritmos Puros](https://yorche3.github.io/programming_languages/ROADMAP/#fase-1--algoritmos-puros--algorithms-pure-) en **Rust**: ordenamientos elementales, estructuras de datos propias, ordenamientos óptimos y distribuidos, y búsqueda.

Los módulos de esta fase trabajan sobre **`Vec` mutables**, que se ordenan *in-place* y se devuelven. Un `Vec<i32>` no admite `null`, así que el caso nulo no es representable en la firma.

---

## 📂 Módulos / Modules

| Módulo | Especificación | Enfoque | Tests | Estado |
|--------|---------------|---------|:-----:|:------:|
| [`naive_sort/`](naive_sort/) | [05_Naive_Sort](https://yorche3.github.io/programming_languages/core/algorithms/05_Naive_Sort/) | `cargo test` + `cargo clippy` | 21 | ✅ |

---

## 📁 Estructura / Structure

```text
algorithms/
└── naive_sort/                  # 05_Naive_Sort
    ├── Cargo.toml               # Paquete y [lib] naive_sort
    ├── Cargo.lock
    ├── .gitignore               # Ignora target/
    ├── src/
    │   ├── lib.rs               # pub mod naive_sort;
    │   └── naive_sort.rs        # 3 funciones del contrato
    ├── tests/
    │   └── naive_sort_tests.rs  # 3 tests × 7 casos
    └── README.md
```

---

## 🛠️ Patrón común / Common Pattern

| Característica | Descripción |
|---------------|-------------|
| **Runtime** | Rust 1.98+ (edición 2024); la librería se compila con `cargo` |
| **CLI** | `cargo test`, ejecutado desde la raíz del proyecto |
| **Andamiaje** | ✅ `cargo init --lib` (comando de la guía), el mismo que usa [`foundations/numbers/`](../foundations/numbers/); genera solo la librería, sin `main` de ejemplo |
| **Framework de tests** | El integrado de Rust (`#[test]` + `assert_eq!`), sin dependencias externas |
| **Runner** | `cargo test` descubre `tests/*.rs` (suite de integración) y los `#[cfg(test)]` de `src/`; no hay `run_tests.rs` |
| **Separación** | `src/` (librería) ↔ `tests/` (suites de integración que consumen la API pública) |
| **Módulo fuente** | Un módulo por fichero (`src/naive_sort.rs`) expuesto desde `src/lib.rs` con `pub mod` |
| **API** | Una función por algoritmo, `pub fn <algoritmo>(arr: Vec<i32>) -> Vec<i32>` |
| **Naming** | `snake_case` idéntico al de la especificación (`selection_sort`), que además es la convención de Rust |
| **Mutabilidad** | El `Vec` se recibe por valor y se reordena *in-place* en su propio búfer; la suite ordena una copia (`to_vec()`) por caso |
| **Nulabilidad** | `Vec<i32>` no admite `null`: el caso nulo no es representable y se omite (la fase todavía no usa `Option`/`Result`) |
| **Mensajes de aserción** | `assert_eq!(actual, expected, "{} should sort {}", algorithm, description)` con el mensaje del contrato |
| **Verificación estática** | `cargo clippy --all-targets -- -D warnings` (clippy trata los warnings como errores) |
| **Artefactos** | `target/` (y `Cargo.lock`, que sí se versiona como en `numbers/`) — `target/` ignorado por el `.gitignore` |
| **Particularidades** | Rangos exclusivos (`0..n-1`, `0..n-i-1`) para los `for` del pseudocódigo; `usize` obliga a reescribir el `j >= 0` de `insertion_sort` como `j > 0 && arr[j-1] > key`; el intercambio es `Vec::swap` |

---

## 🚀 Compilación rápida / Quick Build

```bash
# Naive Sort Tests
cd naive_sort
cargo clippy --all-targets -- -D warnings
cargo test
```

---

## ▶️ Siguiente / Next

👉 Continúa con los módulos pendientes de esta fase en el [Roadmap](https://yorche3.github.io/programming_languages/ROADMAP/).

👉 Continue with the pending modules of this phase in the [Roadmap](https://yorche3.github.io/programming_languages/ROADMAP/).

---

*[← Volver a Core](../README.md)*

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
