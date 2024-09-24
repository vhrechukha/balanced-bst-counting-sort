mod balanced_binary_search_tree;
use balanced_binary_search_tree::{AVLTree, Node};
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

fn insert_and_print(avl_tree: &mut AVLTree, key: i32) {
    let root = avl_tree.root.take();
    avl_tree.root = avl_tree.insert(root, key);
    println!("Tree after inserting {}:", key);
    print_tree(&avl_tree.root, 0, true);
    println!();
}

fn delete_and_print(avl_tree: &mut AVLTree, key: i32) {
    let root = avl_tree.root.take();
    avl_tree.root = avl_tree.delete(root, key);
    println!("Tree after deleting {}:", key);
    print_tree(&avl_tree.root, 0, true);
    println!();
}

fn print_tree(node: &Option<Box<Node>>, indent: usize, is_left: bool) {
    if let Some(n) = node {
        println!(
            "{:indent$}{}{}",
            "",
            if is_left { "L-- " } else { "R-- " },
            n.key,
            indent = indent
        );

        // Recursively print the left and right children with increased indentation
        if n.left.is_some() || n.right.is_some() {
            print_tree(&n.left, indent + 4, true); // Left child
            print_tree(&n.right, indent + 4, false); // Right child
        }
    }
}

fn main() {
    println!("1. Simple AVL test");
    let mut avl_tree1: AVLTree = AVLTree::new();

    insert_and_print(&mut avl_tree1, 10);
    insert_and_print(&mut avl_tree1, 20);
    insert_and_print(&mut avl_tree1, 30);
    insert_and_print(&mut avl_tree1, 40);
    insert_and_print(&mut avl_tree1, 50);
    insert_and_print(&mut avl_tree1, 25);

    delete_and_print(&mut avl_tree1, 50);
    delete_and_print(&mut avl_tree1, 40);
    delete_and_print(&mut avl_tree1, 20);
    delete_and_print(&mut avl_tree1, 10);

    delete_and_print(&mut avl_tree1, 30);
    delete_and_print(&mut avl_tree1, 25);

    println!("2. Dataset AVL test");

    let mut rng = rand::thread_rng();
    let mut avl_tree2 = AVLTree::new();

    let mut dataset: Vec<i32> = (0..100).map(|_| rng.gen_range(1..1000)).collect();

    let mut root = avl_tree2.root.take();

    // Measure insert time
    let start_insert = Instant::now();
    for &value in &dataset {
        root = avl_tree2.insert(root, value);
    }
    avl_tree2.root = root;

    let duration_insert = start_insert.elapsed();
    println!("Time taken to insert 100 elements: {:?}", duration_insert);

    // Shuffle dataset and measure find time
    dataset.shuffle(&mut rng);
    let start_find = Instant::now();
    for &value in &dataset {
        avl_tree2.find(&avl_tree2.root, value);
    }
    let duration_find = start_find.elapsed();
    println!("Time taken to find 100 elements: {:?}", duration_find);

    // Measure delete time
    root = avl_tree2.root.take();
    let start_delete = Instant::now();
    for &value in &dataset {
        root = avl_tree2.delete(root, value);
    }
    avl_tree2.root = root;

    let duration_delete = start_delete.elapsed();
    println!("Time taken to delete 100 elements: {:?}", duration_delete);
}
