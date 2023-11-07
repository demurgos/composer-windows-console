use dialoguer::Select;

fn main() {
    let items = vec!["foo", "bar", "baz"];

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    println!("You chose: {}", items[selection]);
}
