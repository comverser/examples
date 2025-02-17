use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug)]
struct Node {
    data: Cell<i32>,
    points: Option<Rc<RefCell<Node>>>,
}

fn main() {
    // Rc::new() is used to create a reference-counted smart pointer
    let node3 = Rc::new(RefCell::new(Node {
        data: Cell::new(10),
        points: None,
    }));

    dbg!(Rc::strong_count(&node3));

    let node2 = Rc::new(RefCell::new(Node {
        data: Cell::new(20),
        points: Some(Rc::clone(&node3)),
    }));

    dbg!(Rc::strong_count(&node3));
    drop(node2);

    let node1 = Rc::new(RefCell::new(Node {
        data: Cell::new(30),
        points: Some(Rc::clone(&node3)),
    }));

    // node1.data.set(888);
    node1.borrow_mut().data.set(888);

    // if let Some(ref node) = node1.points {
    //     node.data.set(1000);
    // }

    dbg!(Rc::strong_count(&node3));

    println!("Before Node1: {:?}", node1.borrow());
    println!("Before Node1: {:?}", node3.borrow());

    node1.borrow_mut().points = None;
    node3.borrow_mut().points = Some(Rc::clone(&node1));

    println!("After Node1: {:?}", node1.borrow());
    println!("After Node1: {:?}", node3.borrow());

    dbg!(node1);
}
