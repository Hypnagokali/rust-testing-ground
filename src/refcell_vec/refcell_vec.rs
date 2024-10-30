use std::cell::RefCell;
use std::rc::Rc;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

const TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(10, 0, 0);

#[derive(Clone, Debug)]
pub struct Fab<'a> {
    code: &'a str,
    executed: NaiveDateTime,
}


pub fn mutate_vec(input: Vec<Rc<RefCell<Fab>>>) -> Vec<Rc<RefCell<Fab>>> {
    if input.len() < 2 {
        return input;
    }

    let mut res = Vec::new();

    for i in 0..input.len() {
        let current = Rc::clone(&input[i]);
        if i == input.len() - 1 {
            res.push(current);
        } else if current.borrow().code != input.get(i + 1).unwrap().borrow().code {
            res.push(current);
        } else {
            current.borrow_mut().executed = input.get(i + 1).unwrap().borrow().executed;
        }
    }

    res

}

#[test]
fn simple_ref_cell_test() {
    let mut input = Vec::new();

    input.push (Rc::new(RefCell::new(Fab {
        code: "123",
        executed: NaiveDate::from_ymd_opt(2024, 5, 5).unwrap().and_time(TIME.unwrap())
    })));
    input.push (Rc::new(RefCell::new(Fab {
        code: "123",
        executed: NaiveDate::from_ymd_opt(2024, 5, 6).unwrap().and_time(TIME.unwrap())
    })));
    input.push (Rc::new(RefCell::new(Fab {
        code: "123",
        executed: NaiveDate::from_ymd_opt(2024, 5, 7).unwrap().and_time(TIME.unwrap()),
    })));
    input.push (Rc::new (RefCell::new(Fab {
        code: "1234",
        executed: NaiveDate::from_ymd_opt(2024, 5, 8).unwrap().and_time(TIME.unwrap()),
    })));

    let res = mutate_vec(input);

    println!("{res:?}");
    assert_eq!(res.len(), 2);
    assert_eq!(res.get(1).unwrap().borrow().code, "1234");
    assert_eq!(res.get(0).unwrap().borrow().executed, NaiveDate::from_ymd_opt(2024, 5, 7).unwrap().and_time(TIME.unwrap()));
}