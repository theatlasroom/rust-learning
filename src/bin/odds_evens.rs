// roll a lot of dice and see how many odds / evens we get
extern crate rand;

fn main() {
    print!("Starting\n");

    let mut i = 0;

    let (mut odd, mut even) = (0,0);

    loop {
        let (roll_type, roll) = roll_dice();
        match roll_type {
            DiceRoll::Odd => odd += 1,
            DiceRoll::Even => even += 1,
        }

        print!("New number {:?}\n", roll);
        if i > 10 {
            break;
        }
        i += 1;
    }

    print!("odds {:?} vs evens {:?}\n", odd, even);
}

enum DiceRoll {
    Odd,
    Even,
}

fn roll_dice() -> (DiceRoll, usize) {
    let roll: usize = random(6);
    let is_odd = |num: usize| -> bool { if (num % 2) > 0 { true } else { false } };
    if is_odd(roll) {
        (DiceRoll::Odd, roll)
    }
    else {
        (DiceRoll::Even, roll)
    }
}

// fn roll_dice() -> u32 {
//     random(6)
// }

// use rand::distributions::{IndependentSample,Range};
// // alternative roll dice
// fn roll_dice() -> u32 {
//     let between = Range::new(0, 6);
//     let mut rng = rand::thread_rng();
//     between.ind_sample(&mut rng) + 1
// }

fn random(num: u8) -> usize {
    ((rand::random::<u8>() % num) + 1) as usize
}
