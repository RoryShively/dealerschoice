use rand::seq::SliceRandom;

pub mod actions;

// I thoroughly researched the seemingly trivial desicion to declare this
// as either a static or const str since either would work. Details on the
// difference can be found at 
// https://rust-lang.github.io/rfcs/0246-const-vs-static.html
// 
// I chose const becuase it inlines the str into code instead of needing to
// reference an address as static would
const ACTION_DIALOG_PROMPT: &str = r#"Chose which action you would like to perform
    Enter number of action you want (ex: 1, 2, 3)
1) Add an element to choices
2) Remove an element to choices
3) Randomly select an choice
"#;


fn main() {
    // let mut choices: Vec<String> = Vec::new();
    let mut choices: Vec<String> = Vec::new();


    loop {
        println!("\nCurrent choices: {:?}", choices);

        let mut input = String::new();
        println!("{}", ACTION_DIALOG_PROMPT);
        // use expect instead of unwrap
        // https://users.rust-lang.org/t/how-to-read-an-integer-from-stdin/57538
        let _ = std::io::stdin().read_line(&mut input).unwrap();
        input.pop();  // remove last char (\n)

        match input.as_str() {
            "1" => actions::add::add(&mut choices),
            "2" => println!("remove"),
            "3" => {
                println!("select");
                break;
            },
            _ => println!("incorrect input submitted"),
        }
    }

    // println!("{:?}", choices.choose(&mut rand::thread_rng()));
}
