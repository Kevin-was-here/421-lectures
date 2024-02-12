fn push(&mut self, val: T) {
    let next = unsafe { (*self.root).next };
    let node = Box::into_raw(Box::new(
        Node::new(val, next, self.root))
    );

    unsafe {
        (*next).prev = node; 
        (*self.root).next = node;
    }
}

fn pop(&mut self) -> Option<T> {
    let node = unsafe { (*self.root).next };

    if node == self.root {
        return None;
    }

    let val = unsafe { (*node).elem.clone() };

    //note that this is a scope block
    unsafe { 
        let next = (*node).next;
        (*next).prev = self.root;
        (*self.root).next = next;
        drop(Box::from_raw(node));
    }
    Some(val)
}
