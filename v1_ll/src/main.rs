use core::str;

#[derive(Debug, PartialEq)]
pub struct LinkedList<T: PartialOrd>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();

    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);

    println!("ll = {:?}", ll);
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn pushing_front_and_back() {
        let mut ll = LinkedList::new();
        ll.push_front(11);
        ll.push_front(9);
        ll.push_back(13);

        let ll_control = LinkedList(Some((
            9,
            Box::new(LinkedList(Some((
                11,
                Box::new(LinkedList(Some((13, Box::new(LinkedList(None)))))),
            )))),
        )));

        println!("ll = {:?}, ll_contorl = {:?}", ll, ll_control);

        assert_eq!(ll, ll_control);
    }
}
