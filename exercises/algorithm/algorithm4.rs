/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::{Debug, Display};


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + Display,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Display,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Display,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut root = self.root.as_mut();
        loop {
            match root {
                None => {
                    self.root = Some(Box::new(TreeNode::new(value)));
                    break;
                }
                Some(node) => {
                    if value > node.value {
                        if node.right.is_none() {
                            node.right = Some(Box::new(TreeNode::new(value)));
                            break;
                        } else {
                            root = node.right.as_mut();
                        }
                    } else if value < node.value {
                        if node.left.is_none() {
                            node.left = Some(Box::new(TreeNode::new(value)));
                            break;
                        } else {
                            root = node.left.as_mut();
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut root = self.root.as_ref();
        loop {
            match root {
                None => {
                    return false;
                }
                Some(node) => {
                    println!("node: {}", node.value);
                    if value > node.value {
                        root = node.right.as_ref();
                    } else if value < node.value {
                        root = node.left.as_ref();
                    } else {
                        return true;
                    }
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


