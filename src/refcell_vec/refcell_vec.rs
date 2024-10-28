use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Fab<'a> {
    code: &'a str,
    id: i32
}

pub fn mutate_vec(input: Vec<RefCell<Fab>>) -> Vec<RefCell<Fab>> {
    if input.len() < 2 {
        return input;
    }

    let mut res = Vec::new();

    let current = input[0].clone();
    for i in 1..input.len() {
        // check
        if i == input.len() - 1 || current.borrow().code != input.get(i + 1).unwrap().borrow().code {
            res.push(current.clone())
        } else {
            // here I need a NaiveDateTime, id is just a test
            current.borrow_mut().id = input.get(i + 1).unwrap().borrow().id;
            res.push(current.clone());
        }
    }

    res

}

#[test]
fn simple_ref_cell_test() {
    let mut input = Vec::new();
    input.push (RefCell::new(Fab {
        code: "123",
        id: 1,
    }));
    input.push (RefCell::new(Fab {
        code: "123",
        id: 2,
    }));
    input.push (RefCell::new(Fab {
        code: "123",
        id: 3,
    }));
    input.push (RefCell::new(Fab {
        code: "1234",
        id: 4,
    }));

    let res = mutate_vec(input);

    println!("{res:?}");
    assert_eq!(res.len(), 2);
    assert_eq!(res.get(1).unwrap().borrow().code, "1234");
}