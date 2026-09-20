// Casos de prueba de la especificación 05_Naive_Sort.md
//
// Caso nulo omitido: Rust no admite `null` para `Vec<i32>` y esta fase todavía
// no usa `Option`/`Result` como indicador de fallo, así que una entrada nula no
// es representable en la firma y no hay indicador que comprobar. Se conservan
// los 7 casos de la especificación.
//
// Aislamiento: los tres algoritmos ordenan in-place el vector recibido (por
// valor), así que cada caso ordena una copia `to_vec()` del fixture compartido.

use naive_sort::naive_sort;

const STANDARD_INPUT: [i32; 6] = [5, 2, 9, 1, 5, 6];
const STANDARD_OUTPUT: [i32; 6] = [1, 2, 5, 5, 6, 9];

const SORTED_INPUT: [i32; 5] = [1, 2, 3, 4, 5];
const SORTED_OUTPUT: [i32; 5] = [1, 2, 3, 4, 5];

const REVERSE_INPUT: [i32; 5] = [5, 4, 3, 2, 1];
const REVERSE_OUTPUT: [i32; 5] = [1, 2, 3, 4, 5];

const IDENTICAL_INPUT: [i32; 4] = [7, 7, 7, 7];
const IDENTICAL_OUTPUT: [i32; 4] = [7, 7, 7, 7];

const NEGATIVE_INPUT: [i32; 5] = [3, -1, 4, -5, 0];
const NEGATIVE_OUTPUT: [i32; 5] = [-5, -1, 0, 3, 4];

const SINGLE_INPUT: [i32; 1] = [42];
const SINGLE_OUTPUT: [i32; 1] = [42];

const EMPTY_INPUT: [i32; 0] = [];
const EMPTY_OUTPUT: [i32; 0] = [];

// Tabla de casos: descripción, entrada y salida esperada.
struct SortCase {
    description: &'static str,
    input: &'static [i32],
    expected: &'static [i32],
}

const CASES: [SortCase; 7] = [
    SortCase {
        description: "an unsorted array",
        input: &STANDARD_INPUT,
        expected: &STANDARD_OUTPUT,
    },
    SortCase {
        description: "an already sorted array",
        input: &SORTED_INPUT,
        expected: &SORTED_OUTPUT,
    },
    SortCase {
        description: "a reverse ordered array",
        input: &REVERSE_INPUT,
        expected: &REVERSE_OUTPUT,
    },
    SortCase {
        description: "an array of identical elements",
        input: &IDENTICAL_INPUT,
        expected: &IDENTICAL_OUTPUT,
    },
    SortCase {
        description: "an array with negative numbers",
        input: &NEGATIVE_INPUT,
        expected: &NEGATIVE_OUTPUT,
    },
    SortCase {
        description: "a single element array",
        input: &SINGLE_INPUT,
        expected: &SINGLE_OUTPUT,
    },
    SortCase {
        description: "an empty array",
        input: &EMPTY_INPUT,
        expected: &EMPTY_OUTPUT,
    },
];

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

#[test]
fn selection_sort() {
    assert_sorts_all_cases("selection_sort", naive_sort::selection_sort);
}

#[test]
fn bubble_sort() {
    assert_sorts_all_cases("bubble_sort", naive_sort::bubble_sort);
}

#[test]
fn insertion_sort() {
    assert_sorts_all_cases("insertion_sort", naive_sort::insertion_sort);
}
