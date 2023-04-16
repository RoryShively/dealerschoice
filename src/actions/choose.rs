use rand::seq::SliceRandom;

pub fn choose(choices: &mut Vec<String>) -> Option<&String> {
    choices.choose(&mut rand::thread_rng())
}