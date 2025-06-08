pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

fn main() {
    let request = hello::GreetRequest {
        name: String::from("Alice"),
    };
    println!("Request: {:?}", request);
}
