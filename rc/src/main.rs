use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    _ref_cell();
}

fn _ref_cell() {
    // _ref_cell_messenger();
    // _ref_cell_borrow_checker();
    // _weak_ref();
    // _ref_cell_rc_reference_cycle();
    // _ref_cell_weak_avoid_reference_cycle();
    // _thread();
    _thread_wait_for_to_be_finished();
}

fn _thread_wait_for_to_be_finished() {
    use std::thread;
    use std::time::Duration;
    let spawned = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("hi number {} from main thread before join", i);
        thread::sleep(Duration::from_millis(1));
    }

    // spawned thread will be jointed with the main thread
    spawned.join().unwrap();
    // Following code will be executed after everything finished before join

    for i in 1..10 {
        println!("hi number {} from main thread after join", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn _thread() {
    use std::thread;
    use std::time::Duration;
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // When main thread finished spawned thread will be terminated, eventhough all code is not finished
}

fn _ref_cell_weak_avoid_reference_cycle() {
    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
        pub parent: RefCell<Weak<Node>>,
        pub children: RefCell<Vec<Rc<Node>>>,
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("Dropping Node {:?}", &self);
        }
    }

    let leaf = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf parent {:?}", leaf.parent.borrow().upgrade());
    println!(
        "Leaf strong_count {}, weak_count {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 12,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "Branch strong_count {}, weak_count {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "Leaf strong_count {}, weak_count {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("Leaf parent {:?}", leaf.parent.borrow().upgrade());

    println!(
        "Leaf strong_count {}, weak_count {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

fn _ref_cell_rc_reference_cycle() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match &self {
                List::Cons(_, tail) => Some(tail),
                List::Nil => None,
            }
        }
    }
    impl Drop for List {
        fn drop(&mut self) {
            println!("Droping list {:?}", self);
        }
    }

    use List::*;

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a ref count initial {}", Rc::strong_count(&a));
    println!("a Next item {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(a.clone())));
    println!("a ref count after b creation {}", Rc::strong_count(&a));
    println!("b ref count initial {}", Rc::strong_count(&b));
    println!("b Next item {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = b.clone();
    }

    println!("a ref count after chaning {}", Rc::strong_count(&a));
    println!("b ref count after chaning {}", Rc::strong_count(&b));

    // This line will cause stack overflow because a's tail is b and b's tail is a, it's a infinit loop
    // println!("a Next item {:?}", a.tail());

    // At the end a and b will never be dropped, Yay memory leaks
}

fn _weak_ref() {
    let a = Rc::new(5);
    assert_eq!(*a, 5);
    assert_eq!(Rc::strong_count(&a), 1);
    // Rc::clone increases strong_count
    let b = Rc::clone(&a);
    assert_eq!(Rc::strong_count(&a), 2);
    assert_eq!(Rc::strong_count(&b), 2);

    // Rc::downgrade convert Rc to Weak and increase weak_count which does not affect drop
    let c = Rc::downgrade(&a);
    assert_eq!(Rc::strong_count(&b), 2);
    assert_eq!(Rc::weak_count(&b), 1);

    // This line will not work, we can not use Weak referece directly
    // assert_eq!(*c, 5);
    // We need to upgrage to Rc first
    let d = Weak::upgrade(&c);
    // Weak::upgrage return an Option of Rc<T>, because the reference might be already dropped

    assert!(d.is_some());
    let e: Weak<i32>;
    {
        let a2 = Rc::new(10);
        e = Rc::downgrade(&a2);
        // a2 will be dropped hear
    }
    assert!(Weak::upgrade(&e).is_none());
}

fn _ref_cell_borrow_checker() {
    // Changing with borrow_mut
    let a = RefCell::new(5);
    let mut b = a.borrow_mut();
    assert_eq!(*b, 5);
    *b = 6;
    assert_eq!(*b, 6);
    // If we try to borrow() now it will panic because it is already mutalby borrowed()
    // let c = a.borrow();
    // We can avoid panic using try_borrow
    let c = a.try_borrow();
    assert!(c.is_err());

    // Immutable borrow multiple time
    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    let d = a.borrow();
    assert_eq!(*b, 10);
    assert_eq!(*c, 10);
    assert_eq!(*d, 10);
    // If we try to borrow_mut it will panic at runtime, because it is already immutably borrowed
    // let e = a.borrow_mut();
    // We can avoid panic using try_borrow_mut, which will return a Result
    let e = a.try_borrow_mut();
    assert!(e.is_err());

    // Multiple borrow_mut is not allowed

    let a = RefCell::new(100);
    let b = a.borrow_mut();
    assert_eq!(*b, 100);
    // If we try to borrow_mut again it will panic at runtime
    // let c = a.borrow_mut();
    let d = a.try_borrow_mut();
    assert!(d.is_err());

    // Swap
    let a = RefCell::new(8);
    let b = RefCell::new(18);
    a.swap(&b);
    assert_eq!(*a.borrow(), 18);
    assert_eq!(*b.borrow(), 8);
}

fn _ref_cell_messenger() {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T: Messenger> LimitTracker<'a, T> {
        pub fn new(messenger: &'a T, max: usize) -> Self {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent Warning: You used over 90% of your quota");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You used over 75% of your quota");
            }
        }
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }
    {
        let mock_messanger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messanger, 100);
        limit_tracker.set_value(75);
        assert_eq!(mock_messanger.sent_messages.borrow().len(), 1);
        limit_tracker.set_value(95);
        assert_eq!(mock_messanger.sent_messages.borrow().len(), 2);
    }
}

fn _deref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(&5, y);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            Self(x)
        }
    }
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, y.0);
    assert_eq!(5, *y);

    // Deref coercion on function or method arguments
    fn hello(name: &str) {
        println!("Hello {name}");
    }

    let name = "Bob";
    hello(name);
    let name = MyBox::new("Alice");
    hello(&name);

    let name = MyBox::new(String::from("Maya"));
    hello(&name);
}

fn _box() {
    // Use case
    // 1) Size not known at compile time, ie recursive type
    // 2) Transfer ownership without deep copy
    // 3) Trait object

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Box::new(Cons(5, Box::new(Cons(8, Box::new(Cons(9, Box::new(Nil)))))));
    dbg!(a);
}

fn _rc() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    fn cons(list: Rc<List>, item: i32) -> List {
        Cons(item, list)
    }

    impl List {
        fn cons(self, item: i32) -> List {
            Cons(item, self.as_rc())
        }

        fn as_rc(self) -> Rc<List> {
            Rc::new(self)
        }
    }

    use List::{Cons, Nil};
    let a = Nil.cons(9).cons(8).cons(5).as_rc();

    dbg!(Rc::strong_count(&a));
    {
        let _b = cons(Rc::clone(&a), 9).cons(100);
        dbg!(Rc::strong_count(&a));
        dbg!(_b);
    }
    let _c = Cons(11, Rc::clone(&a));
    dbg!(Rc::strong_count(&a));
}
