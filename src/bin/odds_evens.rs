// roll a lot of dice and see how many odds / evens we get
extern crate rand;

fn main() {
    print!("Starting\n");
    let is_odd = |num: u32| -> bool { if (num % 2) > 0 { true } else { false } };

    let mut roll: u32 = 0;

    let mut i = 0;

    let (mut odd, mut even) = (0,0);

    loop {
        roll = roll_dice();
        match roll {
            1 if is_odd(roll) => odd += 1,
            _ => even += 1,
        }

        print!("New number {:?}\n", roll);
        if i > 10 {
            break;
        }
        i += 1;
    }

    print!("odds {:?} vs evens {:?}\n", odd, even);
}

fn roll_dice() -> u32 {
    random(6)
}

// use rand::distributions::{IndependentSample,Range};
// // alternative roll dice
// fn roll_dice() -> u32 {
//     let between = Range::new(0, 6);
//     let mut rng = rand::thread_rng();
//     between.ind_sample(&mut rng) + 1
// }

fn random(num: u32) -> u32 {
    (rand::random::<u32>() % num) + 1
}
