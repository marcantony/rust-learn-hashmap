type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>
}

pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None
        }
    }

    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            item: item,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    fn pop_link(&mut self) -> Link<T> {
        self.head.take().map(|mut boxed_node| {
            self.head = boxed_node.next.take();
            boxed_node
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_link().map(|node| node.item)
    }
}

// https://rust-unofficial.github.io/too-many-lists/first-drop.html
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_link() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = LinkedList::<i32>::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
