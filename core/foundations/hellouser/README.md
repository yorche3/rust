# Hello, User! — Rust

Implementación de la especificación [02_Hello_User](https://yorche3.github.io/programming_languages/core/foundations/02_Hello_User/) en **Rust**, con un enfoque manual y minimalista.

Solicita un nombre al usuario mediante la entrada estándar y saluda.

---

## 📂 Archivos y estructura / Files & Structure

| Archivo | Propósito |
|---------|-----------|
| [`hellouser.rs`](hellouser.rs) | Código fuente: lee una línea desde `stdin` y muestra un saludo. |

**Estructura de directorios esperada:**

```text
hellouser/
├── hellouser.rs     # Código fuente
└── README.md        # Este archivo
```

---

## 🛠️ Enfoque y construcción / Approach & Build

**ES:** Este programa introduce tres conceptos nuevos respecto a `helloworld`:

1. **Importación de la biblioteca estándar** — `use std::io;` importa el módulo de entrada/salida de Rust.
2. **Entrada estándar mutable** — `String::new()` crea un buffer vacío y `read_line(&mut name)` escribe la línea introducida en él.
3. **Formato de salida** — `println!("Hello, {}!", name)` sustituye `{}` por el contenido de `name`.

**EN:** This program introduces three new concepts compared to `helloworld`:

1. **Standard library import** — `use std::io;` imports Rust's input/output module.
2. **Mutable standard input** — `String::new()` creates an empty buffer and `read_line(&mut name)` writes the user's line into it.
3. **Formatted output** — `println!("Hello, {}!", name)` replaces `{}` with the contents of `name`.

### Inicialización / Initialization

1. Crear la estructura de directorios:

   ```bash
   mkdir -p rust/core/foundations/hellouser
   ```

2. Escribir el archivo `hellouser.rs` con el código fuente.

3. Compilar el archivo con `rustc`.

---

## 📄 Archivos de configuración clave / Key Configuration Files

No se requieren archivos de configuración de Cargo para este ejercicio. El programa se compila directamente con `rustc`.

```rust
use std::io;

fn main() {
   println!("Enter your name: ");
   let mut name = String::new();
   io::stdin().read_line(&mut name);
   println!("Hello, {}!", name);
}
```

| Elemento | Propósito |
|----------|-----------|
| `use std::io;` | Importa las APIs de entrada/salida de la biblioteca estándar. |
| `fn main()` | Función de entrada obligatoria del ejecutable Rust. |
| `let mut name` | Declara un `String` mutable que puede recibir la entrada leída. |
| `io::stdin()` | Obtiene el handle de la entrada estándar del proceso. |
| `read_line(&mut name)` | Lee una línea y la añade al `String`; conserva el salto de línea final. |
| `println!(...)` | Macro de salida con formato y salto de línea. |
| `{}` | Placeholder de formato que recibe el valor de `name`. |

> **ES:** `read_line` devuelve un `Result<usize>`, pero esta implementación no lo consume; por eso `rustc` muestra un warning `unused_must_use`. En código de producción convendría manejar el resultado con `expect`, `unwrap` o `?`.
> **EN:** `read_line` returns a `Result<usize>`, but this implementation does not consume it; therefore `rustc` emits an `unused_must_use` warning. Production code should handle the result with `expect`, `unwrap`, or `?`.

---

## 🚀 Compilación y ejecución / Build & Run

### Requisitos / Requirements

- **Rust toolchain** instalado mediante [rustup](https://rustup.rs/).
- `rustc` 1.98.1 y `cargo` 1.98.1 en el entorno utilizado para esta implementación.

Rust se carga desde los perfiles del shell:

```bash
# ~/.bash_profile y ~/.profile
. "$HOME/.cargo/env"

# ~/.bashrc
. "$HOME/.cargo/env"
```

Para cargarlo en una sesión actual:

```bash
source ~/.bash_profile
# Alternativa directa
source "$HOME/.cargo/env"

rustc --version
cargo --version
```

### Compilar y ejecutar / Build & Run

```bash
cd rust/core/foundations/hellouser
rustc hellouser.rs -o hellouser
./hellouser
```

También se puede probar con entrada redirigida:

```bash
printf 'Ada\n' | ./hellouser
```

### Comprobar sintaxis / Check syntax

Para una comprobación rápida de compilación sin conservar manualmente el ejecutable:

```bash
rustc hellouser.rs -o /tmp/hellouser-rust
```

> **ES:** La versión de `rustc` utilizada no acepta la opción `rustc --check`; compilar el archivo es la comprobación apropiada para este proyecto independiente.
> **EN:** The `rustc` version used here does not accept the `rustc --check` option; compiling the file is the appropriate check for this standalone project.

### Salida esperada / Expected output

Con la entrada `Ada`, la salida exacta del código actual es:

```text
Enter your name: 
Ada
Hello, Ada
!
```

> **ES:** `read_line` conserva el `\n` introducido por Enter. Como el saludo usa `println!` después de insertar `name`, el signo `!` aparece en la línea siguiente. El README refleja el comportamiento actual del archivo sin modificarlo.
> **EN:** `read_line` preserves the `\n` entered by pressing Enter. Because the greeting uses `println!` after inserting `name`, the `!` appears on the next line. This README reflects the current file's behavior without modifying it.

---

## 📝 Notas de implementación / Implementation Notes

- **ES:** Rust requiere una función `main` como punto de entrada del ejecutable.
- **EN:** Rust requires a `main` function as the executable entry point.
- **ES:** `read_line` recibe una referencia mutable (`&mut name`) para rellenar el buffer existente.
- **EN:** `read_line` receives a mutable reference (`&mut name`) to fill the existing buffer.
- **ES:** El ejecutable generado (`hellouser`) es un artefacto de compilación y no forma parte del código fuente del ejercicio.
- **EN:** The generated executable (`hellouser`) is a build artifact and is not part of the exercise's source code.

---

## 🌐 Otras implementaciones / Other implementations

Este proyecto también está implementado en otros lenguajes. Explora el [repositorio principal](https://github.com/yorche3/programming_languages) para ver todas las versiones.

---

*🌐 [github.com/yorche3/programming_languages](https://github.com/yorche3/programming_languages) · [GitHub Pages](https://yorche3.github.io/programming_languages/)*
