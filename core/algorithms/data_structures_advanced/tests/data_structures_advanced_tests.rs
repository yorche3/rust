// Casos de prueba de la especificación 07_Data_Structures_Advanced.md
//
// Grupos: TreeNode (5 casos), BinaryTree (7) y Graph (11), en ese orden.
// Cada caso es un test con instancia propia y sus pasos en orden.
// Omitido: «Adaptación inmutable (si aplica)» de TreeNode; Rust muta en el
// sitio (`set_left(&mut self, ..)`), no hay variante inmutable que probar.
// Adecuaciones: la ausencia es `Option::None` (`left`, `right`, `neighbors`);
// el éxito y el fallo de las operaciones son `true`/`false`.

use data_structures_advanced::data_structures_advanced::{BinaryTree, Graph, TreeNode};

fn child(value: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(value)))
}

fn tree_with(values: &[i32]) -> BinaryTree {
    let mut tree = BinaryTree::new();
    for &value in values {
        assert!(
            tree.insert(value),
            "BinaryTree insert({value}) should succeed"
        );
    }
    tree
}

fn graph_with(labels: &[i32]) -> Graph {
    let mut graph = Graph::new();
    for &label in labels {
        assert!(
            graph.add_vertex(label),
            "Graph add_vertex({label}) should succeed"
        );
    }
    graph
}

fn assert_tree_contains(tree: &BinaryTree, values: &[i32], case: &str) {
    for &value in values {
        assert!(
            tree.contains(value),
            "{case}: contains({value}) should succeed"
        );
    }
}

fn assert_counts(graph: &Graph, vertices: usize, edges: usize, case: &str) {
    assert_eq!(
        vertices,
        graph.vertex_count(),
        "{case}: vertex_count should be {vertices}"
    );
    assert_eq!(
        edges,
        graph.edge_count(),
        "{case}: edge_count should be {edges}"
    );
}

// ---------------------------------------------------------------------------
// TreeNode
// ---------------------------------------------------------------------------

#[test]
fn tree_node_create() {
    let standard_input = TreeNode::new(10);
    assert_eq!(
        10,
        standard_input.value(),
        "TreeNode create: get_value should return 10"
    );
}

#[test]
fn tree_node_absent_initial_links() {
    let standard_input = TreeNode::new(10);
    assert!(
        standard_input.left().is_none(),
        "TreeNode initial links: get_left should be absent"
    );
    assert!(
        standard_input.right().is_none(),
        "TreeNode initial links: get_right should be absent"
    );
}

#[test]
fn tree_node_link_two_children() {
    let mut parent = TreeNode::new(10);
    parent.set_left(child(20));
    parent.set_right(child(30));
    assert_eq!(
        Some(20),
        parent.left().map(|node| node.value()),
        "TreeNode link two children: left child should be 20"
    );
    assert_eq!(
        Some(30),
        parent.right().map(|node| node.value()),
        "TreeNode link two children: right child should be 30"
    );
}

#[test]
fn tree_node_traverse_each_link() {
    let mut parent = TreeNode::new(10);
    parent.set_left(child(20));
    parent.set_right(child(30));
    let left = parent.left();
    let right = parent.right();
    assert_eq!(
        Some(20),
        left.map(|node| node.value()),
        "TreeNode traverse: left should lead to 20"
    );
    assert_eq!(
        Some(30),
        right.map(|node| node.value()),
        "TreeNode traverse: right should lead to 30"
    );
    assert_eq!(
        Some((true, true)),
        left.map(|node| (node.left().is_none(), node.right().is_none())),
        "TreeNode traverse: left child's links should be absent"
    );
    assert_eq!(
        Some((true, true)),
        right.map(|node| (node.left().is_none(), node.right().is_none())),
        "TreeNode traverse: right child's links should be absent"
    );
}

#[test]
fn tree_node_unlink_children() {
    let mut parent = TreeNode::new(10);
    parent.set_left(child(20));
    parent.set_right(child(30));

    parent.set_left(None);
    assert!(
        parent.left().is_none(),
        "TreeNode unlink: left should be absent after clearing left"
    );
    assert_eq!(
        10,
        parent.value(),
        "TreeNode unlink: value should stay 10 after clearing left"
    );
    assert_eq!(
        Some(30),
        parent.right().map(|node| node.value()),
        "TreeNode unlink: right child 30 should be kept after clearing left"
    );

    parent.set_right(None);
    assert!(
        parent.left().is_none() && parent.right().is_none(),
        "TreeNode unlink: both links should be absent after clearing right"
    );
    assert_eq!(
        10,
        parent.value(),
        "TreeNode unlink: value should stay 10 after clearing right"
    );
}

// ---------------------------------------------------------------------------
// BinaryTree
// ---------------------------------------------------------------------------

#[test]
fn binary_tree_create_empty() {
    let tree = BinaryTree::new();
    assert!(
        tree.is_empty(),
        "BinaryTree create empty: is_empty should be true"
    );
    assert_eq!(0, tree.size(), "BinaryTree create empty: size should be 0");
    assert!(
        !tree.contains(50),
        "BinaryTree create empty: contains(50) should fail"
    );
}

#[test]
fn binary_tree_insert_root() {
    let mut tree = BinaryTree::new();
    assert!(
        tree.insert(50),
        "BinaryTree insert root: insert(50) should succeed"
    );
    assert_eq!(1, tree.size(), "BinaryTree insert root: size should be 1");
    assert!(
        !tree.is_empty(),
        "BinaryTree insert root: is_empty should be false"
    );
    assert!(
        tree.contains(50),
        "BinaryTree insert root: contains(50) should succeed"
    );
}

#[test]
fn binary_tree_insert_lower_and_higher() {
    let tree = tree_with(&[50, 30, 70]);
    assert_eq!(
        3,
        tree.size(),
        "BinaryTree lower and higher: size should be 3"
    );
    assert_tree_contains(&tree, &[50, 30, 70], "BinaryTree lower and higher");
}

#[test]
fn binary_tree_multilevel_branches() {
    let tree = tree_with(&[50, 30, 70, 20, 40]);
    assert_eq!(5, tree.size(), "BinaryTree multilevel: size should be 5");
    assert_tree_contains(&tree, &[50, 30, 70, 20, 40], "BinaryTree multilevel");
}

#[test]
fn binary_tree_missing_value_in_nonempty_tree() {
    let tree = tree_with(&[50]);
    assert!(
        !tree.contains(99),
        "BinaryTree missing value: contains(99) should fail"
    );
    assert_eq!(
        1,
        tree.size(),
        "BinaryTree missing value: size should stay 1"
    );
    assert!(
        tree.contains(50),
        "BinaryTree missing value: contains(50) should still succeed"
    );
}

#[test]
fn binary_tree_idempotent_duplicate() {
    let tree = tree_with(&[50, 30, 70, 30]);
    assert_eq!(3, tree.size(), "BinaryTree duplicate: size should be 3");
    assert_tree_contains(&tree, &[50, 30, 70], "BinaryTree duplicate");
}

#[test]
fn binary_tree_ordered_insertions() {
    let tree = tree_with(&[20, 30, 40, 50, 70]);
    assert_eq!(
        5,
        tree.size(),
        "BinaryTree ordered insertions: size should be 5"
    );
    assert_tree_contains(
        &tree,
        &[20, 30, 40, 50, 70],
        "BinaryTree ordered insertions",
    );
    assert!(
        !tree.contains(99),
        "BinaryTree ordered insertions: contains(99) should fail"
    );
}

// ---------------------------------------------------------------------------
// Graph
// ---------------------------------------------------------------------------

#[test]
fn graph_create_empty() {
    let graph = Graph::new();
    assert!(
        graph.is_empty(),
        "Graph create empty: is_empty should be true"
    );
    assert_counts(&graph, 0, 0, "Graph create empty");
    assert!(
        !graph.has_edge(10, 30),
        "Graph create empty: has_edge(10,30) should fail"
    );
    assert_eq!(
        None,
        graph.neighbors(10),
        "Graph create empty: get_neighbors(10) should fail"
    );
}

#[test]
fn graph_isolated_vertex() {
    let mut graph = Graph::new();
    assert!(
        graph.add_vertex(10),
        "Graph isolated vertex: add_vertex(10) should succeed"
    );
    assert!(
        !graph.is_empty(),
        "Graph isolated vertex: is_empty should be false"
    );
    assert_counts(&graph, 1, 0, "Graph isolated vertex");
    assert_eq!(
        Some(vec![]),
        graph.neighbors(10),
        "Graph isolated vertex: neighbours of 10 should be empty"
    );
}

#[test]
fn graph_nonconsecutive_labels_and_directed_edge() {
    let mut graph = graph_with(&[10, 70]);
    assert!(
        graph.add_edge(10, 70),
        "Graph directed edge: add_edge(10,70) should succeed"
    );
    assert!(
        graph.has_edge(10, 70),
        "Graph directed edge: forward edge should be present"
    );
    assert!(
        !graph.has_edge(70, 10),
        "Graph directed edge: reverse edge should be absent"
    );
    assert_counts(&graph, 2, 1, "Graph directed edge");
}

#[test]
fn graph_duplicate_vertex_preserves_adjacency() {
    let mut graph = graph_with(&[10, 30]);
    assert!(
        graph.add_edge(10, 30),
        "Graph duplicate vertex: add_edge(10,30) should succeed"
    );
    assert!(
        graph.add_vertex(10),
        "Graph duplicate vertex: add_vertex(10) again should succeed"
    );
    assert_counts(&graph, 2, 1, "Graph duplicate vertex");
    assert_eq!(
        Some(vec![30]),
        graph.neighbors(10),
        "Graph duplicate vertex: neighbours of 10 should be [30]"
    );
}

#[test]
fn graph_duplicate_edge() {
    let mut graph = graph_with(&[10, 30]);
    assert!(
        graph.add_edge(10, 30),
        "Graph duplicate edge: first add_edge(10,30) should succeed"
    );
    assert!(
        graph.add_edge(10, 30),
        "Graph duplicate edge: second add_edge(10,30) should succeed"
    );
    assert_counts(&graph, 2, 1, "Graph duplicate edge");
    assert_eq!(
        Some(vec![30]),
        graph.neighbors(10),
        "Graph duplicate edge: neighbours of 10 should be [30]"
    );
}

#[test]
fn graph_missing_endpoint_without_mutation() {
    let mut graph = graph_with(&[10, 30]);
    assert!(
        graph.add_edge(10, 30),
        "Graph missing endpoint: add_edge(10,30) should succeed"
    );
    for (u, v) in [(10, 99), (99, 10), (98, 99), (99, 99)] {
        assert!(
            !graph.add_edge(u, v),
            "Graph missing endpoint: add_edge({u},{v}) should fail"
        );
        assert!(
            !graph.has_edge(u, v),
            "Graph missing endpoint: has_edge({u},{v}) should stay absent"
        );
    }
    assert_counts(&graph, 2, 1, "Graph missing endpoint");
    assert_eq!(
        Some(vec![30]),
        graph.neighbors(10),
        "Graph missing endpoint: neighbours of 10 should stay [30]"
    );
    assert_eq!(
        Some(vec![]),
        graph.neighbors(30),
        "Graph missing endpoint: neighbours of 30 should stay empty"
    );
    assert_eq!(
        None,
        graph.neighbors(98),
        "Graph missing endpoint: get_neighbors(98) should fail"
    );
    assert_eq!(
        None,
        graph.neighbors(99),
        "Graph missing endpoint: get_neighbors(99) should fail"
    );
}

#[test]
fn graph_sorted_neighbours_and_counts() {
    let mut graph = graph_with(&[10, 30, 50, 70]);
    for (u, v) in [(10, 70), (10, 30), (10, 50), (30, 50)] {
        assert!(
            graph.add_edge(u, v),
            "Graph sorted neighbours: add_edge({u},{v}) should succeed"
        );
    }
    let expected = [
        (10, vec![30, 50, 70]),
        (30, vec![50]),
        (50, vec![]),
        (70, vec![]),
    ];
    for (label, neighbours) in expected {
        assert_eq!(
            Some(neighbours),
            graph.neighbors(label),
            "Graph sorted neighbours: neighbours of {label} should match"
        );
    }
    assert_counts(&graph, 4, 4, "Graph sorted neighbours");
}

#[test]
fn graph_missing_edge_query() {
    let graph = graph_with(&[10, 30]);
    assert!(
        !graph.has_edge(10, 30),
        "Graph missing-edge query: has_edge(10,30) should fail"
    );
    assert_counts(&graph, 2, 0, "Graph missing-edge query");
    assert!(
        !graph.is_empty(),
        "Graph missing-edge query: is_empty should be false"
    );
}

#[test]
fn graph_neighbours_of_missing_vertex() {
    let graph = graph_with(&[10]);
    let missing = graph.neighbors(99);
    assert_eq!(
        None, missing,
        "Graph missing vertex: get_neighbors(99) should fail"
    );
    assert_ne!(
        missing,
        graph.neighbors(10),
        "Graph missing vertex: failure should differ from the empty neighbours of 10"
    );
    assert_counts(&graph, 1, 0, "Graph missing vertex");
}

#[test]
fn graph_self_loop_and_duplicate() {
    let mut graph = graph_with(&[10]);
    assert!(
        graph.add_edge(10, 10),
        "Graph self-loop: first add_edge(10,10) should succeed"
    );
    assert!(
        graph.add_edge(10, 10),
        "Graph self-loop: second add_edge(10,10) should succeed"
    );
    assert!(
        graph.has_edge(10, 10),
        "Graph self-loop: has_edge(10,10) should succeed"
    );
    assert_eq!(
        Some(vec![10]),
        graph.neighbors(10),
        "Graph self-loop: neighbours of 10 should be [10]"
    );
    assert_counts(&graph, 1, 1, "Graph self-loop");
}

#[test]
fn graph_neighbour_result_isolation() {
    let mut graph = graph_with(&[10, 30, 50]);
    assert!(
        graph.add_edge(10, 30),
        "Graph isolation: add_edge(10,30) should succeed"
    );
    let saved = graph.neighbors(10);
    let mut copy = graph
        .neighbors(10)
        .expect("Graph isolation: 10 should exist");
    copy.push(99);
    assert_eq!(
        Some(vec![30]),
        graph.neighbors(10),
        "Graph isolation: changing the copy should not change the graph"
    );
    assert!(
        graph.add_edge(10, 50),
        "Graph isolation: add_edge(10,50) should succeed"
    );
    assert_eq!(
        Some(vec![30]),
        saved,
        "Graph isolation: saved result should remain [30]"
    );
    assert_eq!(
        Some(vec![30, 50]),
        graph.neighbors(10),
        "Graph isolation: fresh query should be [30,50]"
    );
    assert_counts(&graph, 3, 2, "Graph isolation");
}
