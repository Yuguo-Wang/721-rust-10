use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let xml_str = r#"
        <Person>
            <name>John Doe</name>
            <age>30</age>
        </Person>
    "#;

    let person: Result<Person, _> = serde_xml_rs::from_str(xml_str);

    match person {
        Ok(person) => println!("Parsed XML: {:?}", person),
        Err(err) => eprintln!("Error parsing XML: {}", err),
    }
}
