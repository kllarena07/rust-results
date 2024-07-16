use rand::{rngs::ThreadRng, Rng};

fn simulate_fetching_data<'a>() -> Result<&'a str, &'a str> {
    let mut rng: ThreadRng = rand::thread_rng();
    let val: i32 = rng.gen_range(0..=1);

    if val == 0 {
        Ok("Data was fetched successfully!")
    } else {
        Err("There was an error fetching the data")
    }
}

fn main() {
    println!("Hello, world!");
}
