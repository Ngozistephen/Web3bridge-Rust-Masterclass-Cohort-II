use std::{any::Any, cell::RefCell, ops::Deref, rc::Rc};
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct Student {
    name: String,
    id: u8,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    use crate::List::{Cons, Nil};

    // box, rc, arc,mutex, refcell
    let our_box = Box::new(21);
    let our_string = Box::new(String::from("Hello"));
    println!("{our_box}");
    println!("{:?}", std::ptr::addr_of!(*our_box).type_id());
    let a = 5;

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("Our list is {list:?}");
    let b = 5;
    println!("{:?}", a.type_id());
    println!("{:?}", b.type_id());

    //treating smart pointers like regular references
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("value five is: {}", *y);

    let main_box = Box::new(String::from("holla"));
    hello(&main_box);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // RC
    let student = Rc::new(Student {
        name: String::from("vec"),
        id: 2,
    });

    let student1 = Rc::clone(&student);
    let student2 = Rc::clone(&student);
    println!("Student-1: {} {}", student1.name, student1.id);
    println!("Student-1: {} {}", student2.name, student2.id);
    println!(
        "Count to numnber of students: {}",
        Rc::strong_count(&student)
    );

    // this won't work because we cannot use binary assignment
    // operator on &i32
    // let mut a = 2;
    // &a += 1;

    // let mut x = 5;
    // let y = &mut x;
    // y += 1;

    let ref_cell: RefCell<i32> = RefCell::new(14);
    println!("Our refcell: {:?}", ref_cell);
    *ref_cell.borrow_mut() += 1;

    println!("Our refcell: {:?}", ref_cell);

    let another_student = Rc::new(RefCell::new(Student {
        name: String::from("vec"),
        id: 2,
    }));

    println!(
        "Student-1: {} {}",
        another_student.borrow().name,
        another_student.borrow().id
    );

    let another_student1 = Rc::clone(&another_student);
    let another_student2 = Rc::clone(&another_student);

    println!("========REFCELL==========");

    println!(
        "Student-2: {:?} {}",
        another_student1.borrow().name,
        another_student1.borrow().id
    );

    // mutating value inside student 2
    another_student2.borrow_mut().name = String::from("hello");

    println!(
        "Student-2: {:?} {}",
        another_student2.borrow().name,
        another_student2.borrow().id
    );
    println!(
        "Count to numnber of our other students: {}",
        Rc::strong_count(&another_student)
    );
}
