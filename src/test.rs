fn main() {
    let _sentence = String::from("slf -ren");

    for i in _sentence.split_whitespace() {
        match i {
            "slf" => {
                match i {
                    "-ren" => {
                        println!("1");
                    }
                    _ => {
                        println!("4");
                    }
                }
                {}
            }
            "in" => {
                println!("2");
            }
            "the" => {
                println!("3");
            }
            _ => {
                println!("4");
            }
        }
    }
}
