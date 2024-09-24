// BST = AVL, named as inventors Adelson-Velsky and Landis, who introduced it in 1962

use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Node {
    pub key: i32,
    pub height: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i32) -> Node {
        Node {
            key,
            height: 1,
            left: None,
            right: None,
        }
    }
}

pub struct AVLTree {
    pub root: Option<Box<Node>>,
}

impl AVLTree {
    pub fn new() -> AVLTree {
        AVLTree { root: None }
    }

    pub fn height(n: &Option<Box<Node>>) -> i32 {
        match n {
            Some(node) => node.height,
            None => 0,
        }
    }

    pub fn balance_factor(n: &Option<Box<Node>>) -> i32 {
        if let Some(node) = n {
            return AVLTree::height(&node.left) - AVLTree::height(&node.right);
        }
        0
    }

    pub fn right_rotate(mut y: Box<Node>) -> Box<Node> {
        let mut x = y.left.take().unwrap();
        let t2 = x.right.take();

        x.right = Some(y);
        x.right.as_mut().unwrap().left = t2;

        x.right.as_mut().unwrap().height = max(
            AVLTree::height(&x.right.as_ref().unwrap().left),
            AVLTree::height(&x.right.as_ref().unwrap().right),
        ) + 1;
        x.height = max(AVLTree::height(&x.left), AVLTree::height(&x.right)) + 1;

        x
    }

    pub fn left_rotate(mut x: Box<Node>) -> Box<Node> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();

        y.left = Some(x);

        y.left.as_mut().unwrap().right = t2;

        y.left.as_mut().unwrap().height = max(
            AVLTree::height(&y.left.as_ref().unwrap().left),
            AVLTree::height(&y.left.as_ref().unwrap().right),
        ) + 1;

        y.height = max(AVLTree::height(&y.left), AVLTree::height(&y.right)) + 1;

        y
    }

    pub fn insert(&mut self, root: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
        if let Some(mut node) = root {
            if key < node.key {
                node.left = self.insert(node.left.take(), key);
            } else if key > node.key {
                node.right = self.insert(node.right.take(), key);
            } else {
                return Some(node);
            }

            node.height = max(AVLTree::height(&node.left), AVLTree::height(&node.right)) + 1;

            let balance = AVLTree::balance_factor(&Some(node.clone()));

            if balance > 1 && key < node.left.as_ref().unwrap().key {
                return Some(AVLTree::right_rotate(node));
            }
            if balance < -1 && key > node.right.as_ref().unwrap().key {
                return Some(AVLTree::left_rotate(node));
            }
            if balance > 1 && key > node.left.as_ref().unwrap().key {
                node.left = Some(AVLTree::left_rotate(node.left.take().unwrap()));
                return Some(AVLTree::right_rotate(node));
            }
            if balance < -1 && key < node.right.as_ref().unwrap().key {
                node.right = Some(AVLTree::right_rotate(node.right.take().unwrap()));
                return Some(AVLTree::left_rotate(node));
            }

            Some(node)
        } else {
            Some(Box::new(Node::new(key)))
        }
    }

    pub fn find(&self, root: &Option<Box<Node>>, key: i32) -> bool {
        match root {
            Some(node) => {
                if node.key == key {
                    true
                } else if key < node.key {
                    self.find(&node.left, key)
                } else {
                    self.find(&node.right, key)
                }
            }
            None => false,
        }
    }

    pub fn delete(&mut self, root: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
        if root.is_none() {
            return None;
        }
    
        let mut node = root.unwrap();
    
        // Step 1: Perform standard BST deletion
        if key < node.key {
            node.left = self.delete(node.left.take(), key);
        } else if key > node.key {
            node.right = self.delete(node.right.take(), key);
        } else {
            // This is the node to be deleted
    
            // Case 1: Node with only one child or no child
            if node.left.is_none() {
                return node.right; // Return right child if exists, else None
            } else if node.right.is_none() {
                return node.left;  // Return left child if exists, else None
            }
    
            // Case 2: Node with two children
            // Get the inorder successor (smallest in the right subtree)
            let temp = Self::find_min(&node.right);
            node.key = temp.key;
    
            // Delete the inorder successor
            node.right = self.delete(node.right.take(), temp.key);
        }
    
        // Step 2: Update the height of the current node
        node.height = 1 + max(Self::height(&node.left), Self::height(&node.right));
    
        // Step 3: Get the balance factor and balance the tree
        let balance = Self::balance_factor(&Some(node.clone()));
    
        // Left Left Case
        if balance > 1 && Self::balance_factor(&node.left) >= 0 {
            return Some(Self::right_rotate(node));
        }
    
        // Left Right Case
        if balance > 1 && Self::balance_factor(&node.left) < 0 {
            node.left = Some(Self::left_rotate(node.left.take().unwrap()));
            return Some(Self::right_rotate(node));
        }
    
        // Right Right Case
        if balance < -1 && Self::balance_factor(&node.right) <= 0 {
            return Some(Self::left_rotate(node));
        }
    
        // Right Left Case
        if balance < -1 && Self::balance_factor(&node.right) > 0 {
            node.right = Some(Self::right_rotate(node.right.take().unwrap()));
            return Some(Self::left_rotate(node));
        }
    
        Some(node)
    }
    
    // Helper function to find the minimum node (leftmost node)
    fn find_min(node: &Option<Box<Node>>) -> Box<Node> {
        let mut current = node.as_ref().unwrap();
        while let Some(ref left) = current.left {
            current = left;
        }
        current.clone()
    }
    
}
