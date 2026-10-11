// Casos de prueba de la especificación 06_Data_Structures_Basics.md
//
// Contrato: cuatro tipos públicos (`Node`, `LinkedList`, `Stack`, `Queue`) y un
// único `Node` compartido. Cada tabla de la especificación se ejecuta como una
// secuencia de operaciones sobre la MISMA instancia, sin recrear objetos para
// cada borde: los escenarios van de lo vacío a lo lleno y vuelven a lo vacío.
//
// Adecuaciones de salida de la fase: las lecturas enteras que pueden fallar
// devuelven -1 (`head`, `peek`, `pop`, `dequeue`); las booleanas devuelven
// `true`/`false` (`delete`, `is_empty`) y `size` devuelve el número de nodos.
// Solo los enlaces son `Option` (`Node::next`), así que la ausencia se
// comprueba con `is_none()`. No se espera ningún pánico.

use data_structures_basics::data_structures_basics::{LinkedList, Node, Queue, Stack};

// ---------------------------------------------------------------------------
// Node
// ---------------------------------------------------------------------------

#[test]
fn node_get_value() {
    let node = Node::new(42);
    assert_eq!(
        42,
        node.value(),
        "Node get_value should return the value given to init"
    );
}

#[test]
fn node_get_next() {
    let node = Node::new(42);
    assert!(
        node.next().is_none(),
        "Node get_next should be an absent link before set_next"
    );
}

#[test]
fn node_set_next() {
    let mut node = Node::new(42);
    node.set_next(Some(Box::new(Node::new(7))));
    assert_eq!(
        Some(7),
        node.next().map(|next| next.value()),
        "Node get_next should see the node linked by set_next"
    );
    node.set_next(None);
    assert!(
        node.next().is_none(),
        "Node set_next should accept an absent link"
    );
}

// ---------------------------------------------------------------------------
// LinkedList
// ---------------------------------------------------------------------------

#[test]
fn linked_list_new() {
    let list = LinkedList::new();
    assert_eq!(0, list.size(), "LinkedList size should be 0 after init");
    assert!(
        list.is_empty(),
        "LinkedList is_empty should be true after init"
    );
    assert_eq!(
        -1,
        list.head(),
        "LinkedList get_head should be the failure indicator when empty"
    );
}

#[test]
fn linked_list_head() {
    let mut list = LinkedList::new();
    assert_eq!(
        -1,
        list.head(),
        "LinkedList get_head should be the failure indicator when empty"
    );
    list.insert_tail(10);
    assert_eq!(
        10,
        list.head(),
        "LinkedList get_head should be 10 after the first insertion"
    );
    list.insert_head(5);
    assert_eq!(
        5,
        list.head(),
        "LinkedList get_head should be the last inserted head"
    );
    assert_eq!(
        5,
        list.head(),
        "LinkedList get_head should not remove the head"
    );
    list.delete(5);
    assert_eq!(
        10,
        list.head(),
        "LinkedList get_head should be 10 after the head is removed"
    );
}

#[test]
fn linked_list_insert_head() {
    let mut list = LinkedList::new();
    list.insert_head(10);
    assert_eq!(
        10,
        list.head(),
        "LinkedList get_head should be 10 after inserting 10 into an empty list"
    );
    assert_eq!(
        1,
        list.size(),
        "LinkedList size should be 1 after one insert_head"
    );
    list.insert_head(5);
    assert_eq!(
        5,
        list.head(),
        "LinkedList get_head should be 5 after inserting 5 at the head"
    );
    assert_eq!(
        2,
        list.size(),
        "LinkedList size should be 2 after two insert_head"
    );
    assert!(
        !list.is_empty(),
        "LinkedList is_empty should be false with two elements"
    );
}

#[test]
fn linked_list_insert_tail() {
    let mut list = LinkedList::new();
    list.insert_tail(10);
    assert_eq!(
        10,
        list.head(),
        "LinkedList get_head should be 10 after the first insert_tail"
    );
    assert_eq!(
        1,
        list.size(),
        "LinkedList size should be 1 after one insert_tail"
    );
    list.insert_tail(20);
    list.insert_tail(30);
    assert_eq!(
        10,
        list.head(),
        "LinkedList get_head should stay 10 when appending at the tail"
    );
    assert_eq!(
        3,
        list.size(),
        "LinkedList size should be 3 after three insert_tail"
    );
    list.insert_head(5);
    assert_eq!(
        5,
        list.head(),
        "LinkedList get_head should be the new head after insert_head on a non empty list"
    );
    assert_eq!(
        4,
        list.size(),
        "LinkedList size should be 4 after mixing insert_head and insert_tail"
    );
}

#[test]
fn linked_list_delete() {
    let mut list = LinkedList::new();
    list.insert_tail(10);
    list.insert_tail(20);
    list.insert_tail(30);
    list.insert_head(5);
    assert_eq!(
        4,
        list.size(),
        "LinkedList size should be 4 before deleting"
    );
    assert!(
        list.delete(5),
        "LinkedList delete should report success when removing the head"
    );
    assert_eq!(
        10,
        list.head(),
        "LinkedList get_head should be 10 after the head is removed"
    );
    assert_eq!(
        3,
        list.size(),
        "LinkedList size should be 3 after the head is removed"
    );
    assert!(
        list.delete(30),
        "LinkedList delete should report success when removing the tail"
    );
    assert_eq!(
        2,
        list.size(),
        "LinkedList size should be 2 after the tail is removed"
    );
    list.insert_tail(40);
    assert_eq!(
        3,
        list.size(),
        "LinkedList size should be 3 after appending to the list whose tail was removed"
    );
    assert!(
        list.delete(40),
        "LinkedList delete should find the element appended after the tail was removed"
    );
    assert_eq!(
        2,
        list.size(),
        "LinkedList size should be 2 after removing that element"
    );
    assert!(
        !list.delete(99),
        "LinkedList delete should report failure for a value that is not stored"
    );
    assert_eq!(
        2,
        list.size(),
        "LinkedList size should not change when delete reports failure"
    );
    assert!(
        list.delete(20),
        "LinkedList delete should report success when removing a middle element"
    );
    assert_eq!(
        1,
        list.size(),
        "LinkedList size should be 1 after removing the middle element"
    );
    assert_eq!(
        10,
        list.head(),
        "LinkedList get_head should be 10 with a single element left"
    );
    assert!(
        list.delete(10),
        "LinkedList delete should report success when removing the only element"
    );
    assert!(
        list.is_empty(),
        "LinkedList is_empty should be true after the only element is removed"
    );
    assert_eq!(
        -1,
        list.head(),
        "LinkedList get_head should be the failure indicator after the list is emptied"
    );
    assert!(
        !list.delete(10),
        "LinkedList delete should report failure on an empty list"
    );
    list.insert_head(1);
    assert_eq!(
        1,
        list.head(),
        "LinkedList get_head should be 1 after reusing an emptied list"
    );
    assert_eq!(
        1,
        list.size(),
        "LinkedList size should be 1 after reusing an emptied list"
    );
}

#[test]
fn linked_list_is_empty() {
    let mut list = LinkedList::new();
    assert!(
        list.is_empty(),
        "LinkedList is_empty should be true for a new list"
    );
    list.insert_tail(10);
    assert!(
        !list.is_empty(),
        "LinkedList is_empty should be false with one element"
    );
    list.delete(10);
    assert!(
        list.is_empty(),
        "LinkedList is_empty should be true after the element is removed"
    );
}

#[test]
fn linked_list_size() {
    let mut list = LinkedList::new();
    assert_eq!(0, list.size(), "LinkedList size should be 0 for a new list");
    list.insert_head(1);
    list.insert_tail(2);
    list.insert_tail(3);
    assert_eq!(
        3,
        list.size(),
        "LinkedList size should be 3 after three insertions"
    );
    list.delete(2);
    assert_eq!(
        2,
        list.size(),
        "LinkedList size should be 2 after one removal"
    );
    list.delete(2);
    assert_eq!(
        2,
        list.size(),
        "LinkedList size should not change when delete reports failure"
    );
}

// ---------------------------------------------------------------------------
// Stack
// ---------------------------------------------------------------------------

#[test]
fn stack_new() {
    let stack = Stack::new();
    assert_eq!(0, stack.size(), "Stack size should be 0 after init");
    assert!(stack.is_empty(), "Stack is_empty should be true after init");
    assert_eq!(
        -1,
        stack.peek(),
        "Stack peek should be the failure indicator when empty"
    );
}

#[test]
fn stack_push() {
    let mut stack = Stack::new();
    stack.push(1);
    assert_eq!(1, stack.size(), "Stack size should be 1 after one push");
    assert_eq!(
        1,
        stack.peek(),
        "Stack peek should be the most recently pushed value"
    );
    stack.push(2);
    stack.push(3);
    assert_eq!(3, stack.size(), "Stack size should be 3 after three pushes");
    assert_eq!(3, stack.peek(), "Stack peek should be 3 after pushing 3");
    assert!(
        !stack.is_empty(),
        "Stack is_empty should be false with three elements"
    );
}

#[test]
fn stack_pop() {
    let mut stack = Stack::new();
    assert_eq!(
        -1,
        stack.pop(),
        "Stack pop should be the failure indicator when empty"
    );
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(
        3,
        stack.pop(),
        "Stack pop should return the last pushed value first"
    );
    assert_eq!(2, stack.size(), "Stack size should be 2 after one pop");
    assert_eq!(
        2,
        stack.pop(),
        "Stack pop should return the next value in LIFO order"
    );
    assert_eq!(
        1,
        stack.pop(),
        "Stack pop should return the first pushed value last"
    );
    assert_eq!(
        0,
        stack.size(),
        "Stack size should be 0 after popping every element"
    );
    assert!(
        stack.is_empty(),
        "Stack is_empty should be true after popping every element"
    );
    assert_eq!(
        -1,
        stack.pop(),
        "Stack pop should be the failure indicator on an empty stack"
    );
    assert_eq!(
        -1,
        stack.peek(),
        "Stack peek should be the failure indicator after the stack is emptied"
    );
    stack.push(9);
    assert_eq!(
        9,
        stack.pop(),
        "Stack pop should work on a stack reused after being emptied"
    );
}

#[test]
fn stack_peek() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(2, stack.peek(), "Stack peek should return the top value");
    assert_eq!(2, stack.size(), "Stack peek should not change the size");
    assert_eq!(2, stack.pop(), "Stack peek should not consume the value");
}

#[test]
fn stack_is_empty() {
    let mut stack = Stack::new();
    assert!(
        stack.is_empty(),
        "Stack is_empty should be true for a new stack"
    );
    stack.push(1);
    assert!(
        !stack.is_empty(),
        "Stack is_empty should be false with one element"
    );
    stack.pop();
    assert!(
        stack.is_empty(),
        "Stack is_empty should be true after the element is popped"
    );
}

#[test]
fn stack_size() {
    let mut stack = Stack::new();
    assert_eq!(0, stack.size(), "Stack size should be 0 for a new stack");
    stack.push(1);
    stack.push(2);
    assert_eq!(2, stack.size(), "Stack size should be 2 after two pushes");
    stack.pop();
    assert_eq!(1, stack.size(), "Stack size should be 1 after one pop");
    stack.pop();
    assert_eq!(
        0,
        stack.size(),
        "Stack size should be 0 after popping every element"
    );
}

// ---------------------------------------------------------------------------
// Queue
// ---------------------------------------------------------------------------

#[test]
fn queue_new() {
    let queue = Queue::new();
    assert_eq!(0, queue.size(), "Queue size should be 0 after init");
    assert!(queue.is_empty(), "Queue is_empty should be true after init");
    assert_eq!(
        -1,
        queue.peek(),
        "Queue peek should be the failure indicator when empty"
    );
}

#[test]
fn queue_enqueue() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    assert_eq!(1, queue.size(), "Queue size should be 1 after one enqueue");
    assert_eq!(
        1,
        queue.peek(),
        "Queue peek should be the first enqueued value"
    );
    queue.enqueue(2);
    queue.enqueue(3);
    assert_eq!(
        3,
        queue.size(),
        "Queue size should be 3 after three enqueues"
    );
    assert_eq!(
        1,
        queue.peek(),
        "Queue peek should stay at the front when enqueuing"
    );
    assert!(
        !queue.is_empty(),
        "Queue is_empty should be false with three elements"
    );
}

#[test]
fn queue_dequeue() {
    let mut queue = Queue::new();
    assert_eq!(
        -1,
        queue.dequeue(),
        "Queue dequeue should be the failure indicator when empty"
    );
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    assert_eq!(
        1,
        queue.dequeue(),
        "Queue dequeue should return the first enqueued value first"
    );
    assert_eq!(2, queue.size(), "Queue size should be 2 after one dequeue");
    assert_eq!(
        2,
        queue.dequeue(),
        "Queue dequeue should return the next value in FIFO order"
    );
    assert_eq!(
        3,
        queue.dequeue(),
        "Queue dequeue should return the last enqueued value last"
    );
    assert_eq!(
        0,
        queue.size(),
        "Queue size should be 0 after dequeuing every element"
    );
    assert!(
        queue.is_empty(),
        "Queue is_empty should be true after dequeuing every element"
    );
    assert_eq!(
        -1,
        queue.dequeue(),
        "Queue dequeue should be the failure indicator on an empty queue"
    );
    assert_eq!(
        -1,
        queue.peek(),
        "Queue peek should be the failure indicator after the queue is emptied"
    );
    queue.enqueue(9);
    assert_eq!(
        9,
        queue.dequeue(),
        "Queue dequeue should work on a queue reused after being emptied"
    );
}

#[test]
fn queue_peek() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    assert_eq!(1, queue.peek(), "Queue peek should return the front value");
    assert_eq!(2, queue.size(), "Queue peek should not change the size");
    assert_eq!(
        1,
        queue.dequeue(),
        "Queue peek should not consume the value"
    );
}

#[test]
fn queue_is_empty() {
    let mut queue = Queue::new();
    assert!(
        queue.is_empty(),
        "Queue is_empty should be true for a new queue"
    );
    queue.enqueue(1);
    assert!(
        !queue.is_empty(),
        "Queue is_empty should be false with one element"
    );
    queue.dequeue();
    assert!(
        queue.is_empty(),
        "Queue is_empty should be true after the element is dequeued"
    );
}

#[test]
fn queue_size() {
    let mut queue = Queue::new();
    assert_eq!(0, queue.size(), "Queue size should be 0 for a new queue");
    queue.enqueue(1);
    queue.enqueue(2);
    assert_eq!(2, queue.size(), "Queue size should be 2 after two enqueues");
    queue.dequeue();
    assert_eq!(1, queue.size(), "Queue size should be 1 after one dequeue");
    queue.dequeue();
    assert_eq!(
        0,
        queue.size(),
        "Queue size should be 0 after dequeuing every element"
    );
}
