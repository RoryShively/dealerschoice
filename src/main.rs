use rand::seq::SliceRandom;

fn main() {
    let mut choices: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        println!("Enter new choice (type \"done\" to exit):");
        let b1 = std::io::stdin().read_line(&mut input).unwrap();
        input.pop();  // remove last char (\n)

        println!("input: {}", input);
        println!("len(input): {}", b1);

        let exit_loop = input.eq("done");
        
        if exit_loop {
            break
        } else {
            choices.push(input.clone());

            println!("choices: {:?}", choices);
            println!("len(choices): {}", choices.len());
            println!("exit_loop: {}", exit_loop);
        } 
    }

    println!("{:?}", choices.choose(&mut rand::thread_rng()));
}
