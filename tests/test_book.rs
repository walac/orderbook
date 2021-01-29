mod input_parser;
mod output_parser;

use input_parser::OrderType;
use orderbook::OrderBook;

use std::path::PathBuf;

#[test]
fn test_book() {
    let f = PathBuf::from(file!());
    let test_dir = f.parent().unwrap();
    let input_file = test_dir.join("input_file.csv");
    let output_file = test_dir.join("output_file.csv");

    let input = input_parser::parse_file(input_file).unwrap();
    let output = output_parser::parse_file(output_file).unwrap();

    for (i, o) in input.iter().zip(output) {
        println!(
            "{}: '{}' vs {}: '{}'",
            i.name, i.description, o.name, o.description
        );

        let mut order_book = OrderBook::new();
        let mut company = "";

        for order in &i.orders {
            match order {
                OrderType::New(ref symbol, ref ord) => {
                    company = symbol;
                    order_book.add(symbol, ord);
                }
                OrderType::Cancel(user_id, order_id) => order_book.cancel(*user_id, *order_id),
            }
        }

        for (generated, expected) in order_book.get_logs(company).unwrap().iter().zip(o.logs) {
            assert_eq!(*generated, expected);
        }
    }
}
