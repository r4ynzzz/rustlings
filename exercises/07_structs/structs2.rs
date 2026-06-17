#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    let user1 = Order {
        name: String::from("Ryan"),
        year: 2002,
        made_by_phone: true,
        made_by_mobile: false,
        made_by_email: false,
        item_number: 1,
        count: 1,
    };

    let user2 = Order {
        name: String::from("Tyrese"),
        made_by_mobile: true,
        item_number: 3,
        count: 3,
        ..user1
    };

    let user3 = Order {
        name: String::from("Thabo"),
        made_by_email: true,
        item_number: 2,
        count: 2,
        ..user2
    };

    println!("{:#?}, {:#?}, {:#?}", user1, user2, user3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template
        };

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
