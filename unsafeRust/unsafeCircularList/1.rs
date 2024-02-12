struct Node<T: std::fmt::Display> {
    next: Link<T>,
    prev: Link<T>,
    elem: T,
    token: Token<T>,
}
//nolonger need Elem in option

type Link<T> = *mut Node<T>;

impl<T: Default + Clone + std::fmt::Display> Node<T> {
    //notice raw pointers are not unsafe (passed in but not dereferenced)
    fn new(elem: T, next: Link<T>, prev: Link<T>) -> Self {
        Self { next, prev, elem: elem.clone(), token: Token::new(Some(elem)) }
    }

    fn empty() -> Self {
        Self {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            elem: Default::default(),
            token: Token::new(None),
        }
    }
}

struct DLCList<T: std::fmt::Display> {
    //root list
    root: Link<T>,
}

impl<T: Clone + Default + std::fmt::Display> DLCList<T> {
    fn new() -> Self {
        Self {
            root: {
                //box a node and convert to raw pointer
                //require a drop implementation to avoid memory leak
                //which is unsafe (shown in 2.rs)
                let me = Box::into_raw(Box::new(Node::<T>::empty()));
                

                //modify the raw pointer is unsafe
                unsafe {
                    (*me).next = me;
                    (*me).prev = me;
                    me as Link<T>
                }
            }
        }
    }
