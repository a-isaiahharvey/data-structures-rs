use alloc::boxed::Box;

#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug, Default, Clone)]
pub struct LinkedListStack<T> {
    top: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Default for Node<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Node {
            data: T::default(),
            next: None,
        }
    }
}

impl<T> LinkedListStack<T>
where
    T: Clone + Default,
{
    /// Creates an empty stack.
    pub fn new() -> Self {
        LinkedListStack { top: None, len: 0 }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, data: T) {
        let mut node = Node::default();

        node.data = data;
        node.next = self.top.take();

        self.top = Some(Box::new(node));
        self.len += 1;
    }

    /// Removes the top element from the stack and returns it, or `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        let top = self.top.take();
        let result = top.map(|mut node| {
            self.top = node.next.take();
            self.len -= 1;
            node.data
        });

        result
    }

    /// Checks if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the length of the stack.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a reference to the top element of the stack.
    /// Returns `None` if the stack is empty.
    pub fn peak(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }
}

impl<T> core::fmt::Display for LinkedListStack<T>
where
    T: Clone + Default + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let mut first = true;
        let mut iter = self.top.clone();
        while let Some(node) = iter.take() {
            if first {
                write!(f, "{}", node.data)?;
                first = false;
            } else {
                write!(f, " -> {}", node.data)?;
            }
            iter = node.next.clone();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, string::String};

    #[test]
    fn test_stack_push() {
        let mut stack = super::LinkedListStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = super::LinkedListStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack = super::LinkedListStack::new();
        assert_eq!(stack.is_empty(), true);
        stack.push(1);
        assert_eq!(stack.is_empty(), false);
        stack.pop();
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn test_stack_len() {
        let mut stack = super::LinkedListStack::new();
        assert_eq!(stack.len(), 0);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        stack.pop();
        assert_eq!(stack.len(), 1);
        stack.pop();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_display() {
        let mut stack = super::LinkedListStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(format!("{}", stack), "3 -> 2 -> 1");
    }

    #[test]
    fn test_stack_display_empty() {
        let stack = super::LinkedListStack::<String>::new();
        assert_eq!(format!("{}", stack), "");
    }

    #[test]
    fn test_stack_display_one() {
        let mut stack = super::LinkedListStack::new();
        stack.push(1);
        assert_eq!(format!("{}", stack), "1");
    }

    #[test]
    fn test_stack_peak() {
        let mut stack = super::LinkedListStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.peak(), Some(&3));
        stack.pop();
        assert_eq!(stack.peak(), Some(&2));
        stack.pop();
        assert_eq!(stack.peak(), Some(&1));
        stack.pop();
        assert_eq!(stack.peak(), None);
    }
}
