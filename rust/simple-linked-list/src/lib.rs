use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node {
            data: element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let h = self.head.take();
        match h {
            Some(val) => {
                self.length -= 1;
                self.head = val.next;
                Some(val.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_data| &boxed_data.data)
    }

    //#[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut ret = Self::new();
        let mut h = self.head;
        while let Some(x) = h {
            ret.push(x.data);
            h = x.next;
        }
        ret
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for i in iter {
            list.push(i.clone());
        }
        list
    }
}

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut list: Vec<T> = vec![];
        let mut node = linked_list.pop();
        while node.is_some() {
            list.insert(0, node.unwrap());
            node = linked_list.pop();
        }
        list
    }
}
