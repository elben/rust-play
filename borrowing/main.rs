
fn main() {
    // The ! means println is a macro
    println!("Learning Rust!");
    refs();
}

// References and borrowing
fn refs() {
    println!("====== References and borrowing");

    // ===================
    // Borrowing and copying
    // ===================

    // v is a vector. Content is on the heap.
    let v = vec![1, 2, 3];

    // These calls *borrow* v.
    println!("{}", v[0]);
    println!("{}", v[0]);

    // Moves v to v2, so v can no longer be accessed.
    let v2 = v;

    // Below is an error, because Vec<i32> does not implement Copy trait, and to pass the object we
    // need to copy the object.
    // use_v(v2);

    // Below throws error because we no longer have access to v:
    // println!("{}", v[0]);
    //
    // But this is OK:
    println!("{}", v2[0]);

    // Below, when incr_int is called, i is copied.
    let i = 1;
    println!("{}", incr_int(i));

    // ===================
    // Mutable references
    // ===================

    // The code will fail in call to incr_int_ref because j is not a mutable reference.
    //
    // let j = 1;
    // incr_int_ref(&j);
    //
    // Instead, we need to declare j as mutable, and also pass in a mutable ref:

    let mut j = 1;
    incr_int_ref(&mut j);
    println!("{} == 2 is {}", j, j == 2);

    // one or more references (&T) to a resource.
    // exactly one mutable reference (&mut T)

    // let y = 10;
    // let yp = &y;

    // println!(yp);

    // ===================
    // Use after free
    // ===================

    // Rust checks that any ref we have is always valid. For example, the following is illegal
    // because once we're out of the inner scope, x is gone, but y is still around.
    //
    // let y: &i32;
    //
    // {
    //   let x = 0;
    //   y = &x;
    // }
    //
    // The following is also illegal, because y is declared before x.
    //
    // let y: &i32;
    // let x = 5;
    // y = &x;
    //
    // To fix, flip the y and x declaration so that x outlives y.
    //
    let x = 5;
    let y: &i32;
    y = &x;
    println!("x outlives y: {}", y);
}

fn use_v(v: Vec<i32>) {
    println!("use_v: {}", v[0]);
}

fn incr_int(i: i32) -> i32 {
    i + 1
}

fn incr_int_ref(i: &mut i32) {
    *i += 1
}

