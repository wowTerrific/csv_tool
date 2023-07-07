use csv_tool::read_greg;

fn main() {
    // let data = vec![
    //     vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
    //     vec!["123".to_string(), "456".to_string(), "789".to_string()],
    //     vec!["yellow".to_string(), "green".to_string(), "blue".to_string()],
    // ];

    // match write_greg("test.greg", data) {
    //     Ok(_) => (),
    //     Err(e) => println!("There was error: {:?}", e),
    // }

    match read_greg("test-bad.greg") {
        Ok(text) => println!("{}", text),
        Err(e) => println!("There was error: {:?}", e),
    }
}

