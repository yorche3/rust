// naive_sort — ordenamientos elementales O(n²).
//
// Especificación: 05_Naive_Sort
//
// Contrato: las tres funciones reciben un vector de enteros por valor y
// devuelven el vector ordenado de menor a mayor. El orden es in-place sobre el
// búfer recibido (se reordena y se devuelve el mismo `Vec`), sin invocar
// `sort`/`sort_by` ni ninguna otra ayuda de ordenamiento de la biblioteca
// estándar.
// Firma: `pub fn selection_sort(arr: Vec<i32>) -> Vec<i32>` (idéntica forma para
// `bubble_sort` e `insertion_sort`). Si el vector está vacío o tiene un solo
// elemento se devuelve sin cambios. No entra en pánico.
//
// Implementación pendiente: la escribe el autor. Esta delegación solo genera el
// esqueleto y las pruebas unitarias.
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

pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    if n < 2 {
        return arr;
    }
    for i in 0..n-1 {
        let mut swapped = false;
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    arr
}

pub fn insertion_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    if n < 2 {
        return arr;
    }
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j-1] > key {
            arr[j] = arr[j-1];
            j -= 1;
        }
        arr[j] = key;
    }
    arr
}