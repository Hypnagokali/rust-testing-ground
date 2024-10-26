fn mutate_customer<Fun>(cust: Customer, f: Fun)
                        -> Customer
where
    Fun: FnOnce(Customer) -> Customer,
{
    let mut mutated_cust = f(cust);
    // do other stuff and then save in cache or db
    mutated_cust.id = 1;
    mutated_cust
}

#[derive(Debug)]
struct Customer {
    id: i32,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_once_closure_to_move_vec_inside_closure_and_move_again() {
        let suffixes = vec!["der", "dritte", "von", "der", "Mühlenburg"];

        let cust = Customer {
            id: 0,
            name: "Hans".to_string(),
        };

        let mutated = mutate_customer(cust, move |mut cust| {
            let m = suffixes.join(" ");

            cust.name = format!("{} {}", cust.name, m);
            cust
        });

        assert_eq!(mutated.name, "Hans der dritte von der Mühlenburg");
        assert_eq!(mutated.id, 1)
    }
}
