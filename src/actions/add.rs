pub fn add(choices: &mut Vec<String>) {
    let mut input = String::new();
    println!("Enter new choice:");
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    input.pop();  // remove last char (\n)

    choices.push(input.clone());
}