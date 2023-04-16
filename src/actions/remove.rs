pub fn remove(choices: &mut Vec<String>) {
    let mut input = String::new();
    println!("Enter choice to remove:");
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    input.pop();  // remove last char (\n)

    let index = match choices.iter().position(|x| *x == input) {
        Some(i) => i,
        None => {
            println!("Item was not found in choices");
            return
        },
    };
    choices.remove(index);
}