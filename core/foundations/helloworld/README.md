# Hello, World! — Rust

Implementación de la especificación [01_Hello_World](https://yorche3.github.io/programming_languages/core/foundations/01_Hello_World/) en **Rust**, con un enfoque manual y minimalista.

---

## 📂 Archivos y estructura / Files & Structure

| Archivo | Propósito |
|---------|-----------|
| [`helloworld.rs`](helloworld.rs) | Código fuente: define `main` e imprime `"Hello, World! from Rust!"` en la salida estándar. |

**Estructura de directorios esperada:**

```text
helloworld/
├── helloworld.rs    # Código fuente
└── README.md        # Este archivo
```

---

## 🛠️ Enfoque y construcción / Approach & Build

**ES:** El proyecto se creó manualmente, sin Cargo ni herramientas de scaffolding. Un único archivo `.rs` es suficiente: Rust es un lenguaje compilado, por lo que el código fuente se compila a un ejecutable antes de ejecutarse.

**EN:** The project was created manually, without Cargo or scaffolding tools. A single `.rs` file is enough: Rust is a compiled language, so the source code is compiled into an executable before it runs.

### Inicialización / Initialization

1. Crear la estructura de directorios:

   ```bash
   mkdir -p rust/core/foundations/helloworld
   ```

2. Escribir el archivo `helloworld.rs` con el código fuente.

3. Compilar el archivo con `rustc`.

---

## 📄 Archivos de configuración clave / Key Configuration Files

No se requieren archivos de configuración de Cargo para este ejercicio. El programa se compila directamente con `rustc`.

```rust
fn main() {
   println!("Hello, World! from Rust!");
}
```

| Elemento | Propósito |
|----------|-----------|
| `fn main()` | Función de entrada obligatoria del ejecutable Rust. |
| `println!(...)` | Macro que escribe el mensaje en `stdout` y añade un salto de línea. |
| `"Hello, World! from Rust!"` | Literal de cadena que se muestra en la salida. |

> **ES:** `println!` es una macro, no una función; el signo `!` identifica las invocaciones de macros en Rust. Para imprimir sin salto de línea se utiliza `print!`.
> **EN:** `println!` is a macro, not a function; the `!` identifies macro invocations in Rust. To print without a newline, use `print!`.

---

## 🚀 Compilación y ejecución / Build & Run

### Requisitos / Requirements

- **Rust toolchain**, normalmente instalado mediante [rustup](https://rustup.rs/).
- `rustc`, el compilador de Rust.
- `cargo`, recomendado para proyectos Rust posteriores, aunque no es necesario para este archivo independiente.

```bash
# Instalar Rust mediante rustup (Linux/macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Recargar el entorno de rustup en la sesión actual
source "$HOME/.cargo/env"

# Verificar la instalación
rustc --version
cargo --version
```

### Compilar y ejecutar / Build & Run

```bash
cd rust/core/foundations/helloworld
rustc helloworld.rs -o helloworld
./helloworld
```

### Comprobar formato y lint (opcional) / Check formatting and lint (optional)

```bash
rustfmt --check helloworld.rs
```

### Salida esperada / Expected output

```text
Hello, World! from Rust!
```

---

## 📝 Notas de implementación / Implementation Notes

- **ES:** Rust requiere una función `main` como punto de entrada de un ejecutable.
- **EN:** Rust requires a `main` function as the entry point of an executable.
- **ES:** `rustc` genera un ejecutable nativo; el binario `helloworld` se crea en el directorio actual con el comando mostrado.
- **EN:** `rustc` generates a native executable; the `helloworld` binary is created in the current directory by the command shown.
- **ES:** El binario generado es un artefacto de compilación y no forma parte del código fuente del ejercicio.
- **EN:** The generated binary is a build artifact and is not part of the exercise's source code.

---

## 🌐 Otras implementaciones / Other implementations

Este proyecto también está implementado en otros lenguajes. Explora el [repositorio principal](https://github.com/yorche3/programming_languages) para ver todas las versiones.

---

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
