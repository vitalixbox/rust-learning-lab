use std::rc::Rc;
use std::sync::Arc;

pub fn run() {
    println!("Chapter 4: Ownership and Moves");

    ownership_and_scope();
    ownership_trees();
    moves();
    moves_in_assignments();
    moves_and_control_flow();
    moves_and_indexed_content();
    moving_option_fields();
    copy_types();
    shared_ownership_with_rc();
    shared_ownership_with_arc();
}

fn ownership_and_scope() {
    // A variable owns its value, including any heap allocation the value owns.
    let mut padovan = vec![1, 1, 1];
    for index in 3..10 {
        let next = padovan[index - 3] + padovan[index - 2];
        padovan.push(next);
    }
    assert_eq!(padovan, [1, 1, 1, 2, 2, 3, 4, 5, 7, 9]);

    // A Box owns its heap-allocated referent.
    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{point:?}");
        assert_eq!(label, "(0.625, 0.5)");
    } // point, label, and their heap allocations are dropped here.

    // drop removes a value from the ownership tree immediately.
    let temporary = String::from("finished with this value");
    assert!(!temporary.is_empty());
    drop(temporary);
    // temporary.len(); // error: temporary was moved into drop.
}

#[derive(Debug)]
struct Person {
    name: String,
    birth: i32,
}

#[allow(clippy::vec_init_then_push)] // Kept to show each value moving into the vector.
fn ownership_trees() {
    // The vector owns its Persons, and each Person owns its String.
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    let names: Vec<&str> = composers
        .iter()
        .map(|composer| composer.name.as_str())
        .collect();
    assert_eq!(names, ["Palestrina", "Dowland", "Lully"]);
    assert_eq!(
        composers.iter().map(|person| person.birth).sum::<i32>(),
        4720
    );
} // Dropping composers recursively drops every owned value.

fn moves() {
    let s = noodle_names();
    let t = s;
    assert_eq!(t, ["udon", "ramen", "soba"]);
    // let u = s; // error: assigning to t moved the value out of s.

    // clone performs the explicit deep copies that assignment does not.
    let s = noodle_names();
    let mut t = s.clone();
    let u = s.clone();
    t[0].push('!');
    assert_eq!(s[0], "udon");
    assert_eq!(t[0], "udon!");
    assert_eq!(u[0], "udon");

    // Function arguments and return values move ownership too.
    let owner = String::from("transferred");
    let (owner, length) = measure_and_return(owner);
    assert_eq!(owner, "transferred");
    assert_eq!(length, 11);

    // Constructing a compound value moves values into its fields.
    let name = String::from("Palestrina");
    let composer = Person { name, birth: 1525 };
    assert_eq!(composer.name, "Palestrina");
    // name.len(); // error: name moved into composer.
}

fn noodle_names() -> Vec<String> {
    vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]
}

fn measure_and_return(text: String) -> (String, usize) {
    let length = text.len();
    (text, length)
}

fn moves_in_assignments() {
    let mut name = "Govinda".to_string();
    assert_eq!(name, "Govinda");
    name = "Siddhartha".to_string(); // The old String is dropped first.
    assert_eq!(name, "Siddhartha");

    let mut name = "Govinda".to_string();
    let original_name = name;
    name = "Siddhartha".to_string(); // name is uninitialized, so nothing is dropped.
    assert_eq!(original_name, "Govinda");
    assert_eq!(name, "Siddhartha");
}

fn moves_and_control_flow() {
    // Moving in both branches is valid because only one branch can run.
    let numbers = vec![10, 20, 30];
    let use_first_branch = true;
    let result = if use_first_branch {
        sum_vector(numbers)
    } else {
        numbers.into_iter().product()
    };
    assert_eq!(result, 60);
    // consume_vector(numbers); // error: either branch has already moved numbers.

    // A loop may move a value if every iteration replaces it before continuing.
    let mut numbers = vec![10, 20, 30];
    let mut iterations = 2;
    while iterations > 0 {
        assert_eq!(consume_vector(numbers), 3);
        numbers = vec![40, 50, 60];
        iterations -= 1;
    }
    assert_eq!(numbers, [40, 50, 60]);

    // This version would fail because a later iteration could reuse a moved value:
    // while condition() {
    //     consume_vector(numbers);
    // }
}

fn consume_vector(numbers: Vec<i32>) -> usize {
    numbers.len()
}

fn sum_vector(numbers: Vec<i32>) -> i32 {
    numbers.into_iter().sum()
}

fn moves_and_indexed_content() {
    let mut values: Vec<String> = (101..106).map(|number| number.to_string()).collect();

    // let third = values[2]; // error: indexing cannot leave a Vec element uninitialized.

    // pop removes and returns the final element.
    let fifth = values.pop().expect("vector should not be empty");
    assert_eq!(fifth, "105");

    // swap_remove fills the removed element's position with the final element.
    let second = values.swap_remove(1);
    assert_eq!(second, "102");

    // replace moves out one value while putting another in its place.
    let third = std::mem::replace(&mut values[2], "substitute".to_string());
    assert_eq!(third, "103");
    assert_eq!(values, ["101", "104", "substitute"]);

    // Iterating by value consumes a collection and moves out every element safely.
    let mottos = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    let mut exclaimed = Vec::new();
    for mut motto in mottos {
        motto.push('!');
        exclaimed.push(motto);
    }
    assert_eq!(exclaimed, ["liberté!", "égalité!", "fraternité!"]);
    // mottos.len(); // error: the for loop consumed mottos.
}

#[derive(Debug)]
struct OptionalPerson {
    name: Option<String>,
    birth: i32,
}

#[allow(clippy::mem_replace_option_with_none)] // Compares the general operation with Option::take.
fn moving_option_fields() {
    let mut composer = OptionalPerson {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    };

    // composer.name cannot be moved through an indexed owner, but None can replace it.
    let first_name = std::mem::replace(&mut composer.name, None);
    assert_eq!(first_name.as_deref(), Some("Palestrina"));
    assert_eq!(composer.name, None);

    // Option::take is the concise form of replacing a value with None.
    composer.name = Some("Giovanni".to_string());
    let next_name = composer.name.take();
    assert_eq!(next_name.as_deref(), Some("Giovanni"));
    assert_eq!(composer.name, None);
    assert_eq!(composer.birth, 1525);
}

#[derive(Copy, Clone)]
struct Label {
    number: u32,
}

fn copy_types() {
    let string1 = "somnambulance".to_string();
    let string2 = string1;
    assert_eq!(string2, "somnambulance");
    // string1.len(); // error: String is moved, not copied.

    let number1 = 36_i32;
    let number2 = number1;
    assert_eq!(number1, 36); // i32 implements Copy, so number1 is still initialized.
    assert_eq!(number2, 36);

    // Tuples and arrays containing only Copy values are Copy too.
    let coordinates = (10, 20);
    let copied_coordinates = coordinates;
    assert_eq!(coordinates, (10, 20));
    assert_eq!(copied_coordinates, (10, 20));

    // User-defined types move by default, even when all their fields are Copy.
    struct NonCopyLabel {
        number: u32,
    }
    let label = NonCopyLabel { number: 3 };
    let moved_label = label;
    assert_eq!(moved_label.number, 3);
    // assert_eq!(label.number, 3); // error: label was moved.

    // Deriving Copy and Clone makes assignment copy this eligible type.
    let label = Label { number: 3 };
    let copied_label = label;
    assert_eq!(label_text(label), "STAMP: 3");
    assert_eq!(copied_label.number, 3);

    // Copy cannot be derived when a field owns a non-Copy value:
    // #[derive(Copy, Clone)]
    // struct StringLabel { name: String }
}

fn label_text(label: Label) -> String {
    format!("STAMP: {}", label.number)
}

fn shared_ownership_with_rc() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t = Rc::clone(&s);
    let u = Rc::clone(&s);

    assert_eq!(Rc::strong_count(&s), 3);
    assert!(Rc::ptr_eq(&s, &t));
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    assert_eq!(format!("{u} noodles"), "shirataki noodles");

    drop(u);
    assert_eq!(Rc::strong_count(&s), 2);

    // s.push_str(" noodles"); // error: a shared Rc referent cannot be mutable.
}

fn shared_ownership_with_arc() {
    // Arc is an atomically reference-counted owner that can cross thread boundaries.
    let shared = Arc::new("thread-safe noodles".to_string());
    let worker_owner = Arc::clone(&shared);
    assert_eq!(Arc::strong_count(&shared), 2);

    let worker = std::thread::spawn(move || worker_owner.contains("noodles"));
    assert!(worker.join().expect("worker thread should finish"));
    assert_eq!(Arc::strong_count(&shared), 1);
}
