/*
    Description:
        internal representation of a syntax tree

    Author: 0xca7
*/

use std::fmt;

extern crate fnv;
use fnv::FnvHash;

/// a node in a derivation tree 
/// this node contains the node `value` and a list of `children`,
/// the children are child nodes of the node
#[derive(Debug)]
pub struct TreeNode {
    /// the value stored in this node
    pub value: String,
    /// a list of child nodes
    /// if None, the symbol is a terminal
    pub children: Option<Vec<TreeNode>>,
}

impl TreeNode {

    /// create a new tree node
    pub fn new(v: &String) -> TreeNode {
        TreeNode {
            value: v.clone(),
            children: None,
        }
    } // pub fn new

    /// insert a new child item, with a `value`
    /// if no children exist, this function creates a vector
    /// to hold the children
    pub fn insert_child(&mut self, value: &String) {
        if self.children.is_none() {
            self.children = Some(Vec::new());
        }
        self.children
            .as_mut().unwrap().push(TreeNode::new(&value.clone()));
    }

    /// when a syntax tree is built and by derivation, we can 
    /// get all leaf elements to form a sentence produced by the
    /// grammar used to generate the tree 
    pub fn build(&self, s: &mut String) {

        if self.children.is_none() {
            s.push_str(&self.value);
        }

        if self.children.is_some() {
            // now, derive further (unwrap safe, tree has children)
            for child in self.children.as_ref().unwrap() {
                //print!("child: {:?}\n", child);
                child.build(s);
            }

        }
    } // fn build

    /// hash the syntax tree to ensure it is unique
    pub fn hash(&self) -> u64 {
        let mut s = String::new();
        let mut fnv = FnvHash::new();
        self.collect(&mut s);
        fnv.hash(s.as_bytes())
    }

    /// create one string from all the elements in the
    /// tree by pre-order traversal, for hashing
    fn collect(&self, s: &mut String) {

        s.push_str(&self.value);
        if self.children.is_some() {
            for child in self.children.as_ref().unwrap() {
                child.collect(s);
            }
        } // traverse 

    }

}

/// currently unimplemented, will be in future
impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unimplemented")
    }
}

