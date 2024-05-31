use rand::Rng;

// Automatically guesses a value
pub enum Input {
    Higher = 0,
    Lower = 1,
    None = 2
}

pub fn Guess(last: i32, input: Input, max: i32)->i32 {
    match input {
        Input::Higher => {
            return last + 1;
        },
        Input::Lower  => {
            return last - 1;
        },
        Input::None  => {
            return rand::thread_rng().gen_range(1..=max);
        }
    }
}