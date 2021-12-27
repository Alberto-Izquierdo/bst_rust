fn main() {
    let tree = BST::new();
    let tree = tree.insert(5);
    let tree = tree.insert(2);
    let tree = tree.insert(1);
    let tree = tree.insert(3);
    let tree = tree.insert(7);
    let tree = tree.remove(5);
    tree.print();
    println!("Hello, world!");
}

struct Node<T: std::cmp::PartialOrd + std::fmt::Display + Copy> {
    value: T,
    left: Box<Option<Node<T>>>,
    right: Box<Option<Node<T>>>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Display + Copy> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    fn insert(self, node: Node<T>) -> Node<T> {
        if node.value < self.value {
            let value_to_insert = if self.left.is_none() {
                Box::new(Some(node))
            } else {
                Box::new(Some(self.left.unwrap().insert(node)))
            };
            Node {
                value: self.value,
                left: value_to_insert,
                right: self.right,
            }
        } else if node.value > self.value {
            let value_to_insert = if self.right.is_none() {
                Box::new(Some(node))
            } else {
                Box::new(Some(self.right.unwrap().insert(node)))
            };
            Node {
                value: self.value,
                left: self.left,
                right: value_to_insert,
            }
        } else {
            self
        }
    }

    fn remove(self, value: T) -> Box<Option<Node<T>>> {
        if value == self.value {
            if self.left.is_none() && self.right.is_none() {
                Box::new(None)
            } else if self.left.is_none() {
                self.right
            } else if self.right.is_none() {
                self.left
            } else {
                let result = self.right.unwrap().remove_min_child_and_get_value();
                Box::new(Some(Node {
                    value: result.1,
                    left: self.left,
                    right: result.0,
                }))
            }
        } else {
            if value > self.value {
                if self.right.is_some() {
                    Box::new(Some(Node {
                        value: self.value,
                        left: self.left,
                        right: self.right.unwrap().remove(value),
                    }))
                } else {
                    Box::new(Some(self))
                }
            } else {
                if self.left.is_some() {
                    Box::new(Some(Node {
                        value: self.value,
                        left: self.left.unwrap().remove(value),
                        right: self.right,
                    }))
                } else {
                    Box::new(Some(self))
                }
            }
        }
    }

    fn remove_min_child_and_get_value(self) -> (Box<Option<Node<T>>>, T) {
        if self.left.is_some() {
            let result = self.left.unwrap().remove_min_child_and_get_value();
            (
                Box::new(Some(Node {
                    value: self.value,
                    left: result.0,
                    right: self.right,
                })),
                result.1,
            )
        } else {
            (Box::new(None), self.value)
        }
    }

    fn print(&self, indentation: i32) {
        if indentation != 0 {
            println!(
                "{}{}{}",
                "| ".repeat(indentation as usize - 1),
                "|-",
                self.value
            );
        } else {
            println!("{}", self.value);
        }
        if self.left.is_some() {
            self.left.as_ref().as_ref().unwrap().print(indentation + 1);
        }
        if self.right.is_some() {
            self.right.as_ref().as_ref().unwrap().print(indentation + 1);
        }
    }
}

struct BST<T: std::cmp::PartialOrd + std::fmt::Display + Copy> {
    root: Option<Node<T>>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Display + Copy> BST<T> {
    fn new() -> BST<T> {
        BST { root: None }
    }

    fn insert(self, value: T) -> BST<T> {
        BST {
            root: self.root.map_or(Some(Node::new(value)), |root| {
                Some(root.insert(Node::new(value)))
            }),
        }
    }

    fn remove(self, value: T) -> BST<T> {
        if self.root.is_some() {
            let result = self.root.unwrap().remove(value);
            return BST { root: *result };
        } else {
            self
        }
    }

    fn print(&self) {
        self.root.as_ref().unwrap().print(0);
    }
}