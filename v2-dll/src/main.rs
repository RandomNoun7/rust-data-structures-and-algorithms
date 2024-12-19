use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}
#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            // create new front object
            Some(r) => {
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // tell the first object this is now in front of it
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // put this in front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            // create new back object
            Some(r) => {
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                // tell the last object this is now in behind it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                // put this in front
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: None,
                    next: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}

fn main() {
    let mut dl = DbList::new();

    dl.push_front(6);
    dl.push_back(11);
    dl.push_front(5);
    dl.push_back(15);
    println!("dl = {:?}", dl);
}
