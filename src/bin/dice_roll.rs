// Dice rolling game
// Uses a naive random function to generate a random number in the range of possible values
use std::time::{Instant, Duration};
use std::thread::sleep;

fn main() {
    let start_time: Instant = Instant::now();
    let values = vec![1,2,3,4,5,6];
    println!("{} possible values {:?}", values.len(), values);

    // just keep on rollin'
    loop {
        sleep(Duration::from_secs(3));
        let arr = values.clone();
        let dice_roll = roll(start_time, arr);
        println!("Dice was rolled, it returned {}", dice_roll);
    }
}

// fn fill_array(max: usize) -> Vec<i32> {
//     // returns a copy of the array that we populate
//     let mut arr = vec![0; max];
//     for (index, value) in (0..max).enumerate() {
//         arr[index] = (value + 1) as i32;
//     }
//     return arr;
// }

fn duration_to_ms(time: Duration) -> u64 {
    // convert the duration to ms
    // TODO: eventually move this to a utils crate/lib
    let nanos =  time.subsec_nanos() as u64;
    return (time.as_secs()*1000) + nanos/1000000;
}

fn current_time(start_time: Instant) -> u64 {
    // return the current time elapsed since we started as milliseconds
    return duration_to_ms(start_time.elapsed());
}

fn is_even(timestamp: u64) -> bool {
    return if timestamp % 2 == 0 { true } else { false };
}

fn roll(start_time: Instant, mut possible_values: Vec<i32>) -> i32 {
    let seed = if is_even(current_time(start_time)) { 1 } else { 2 };
    // let mut dice_roll = 0;
    let mut dice_roll = 0;
    loop {
        let time = current_time(start_time);
        let len = possible_values.len();
        if len < 2 {
            dice_roll = possible_values[0];
            break;
        }

        let middle_index = len / 2;
        if is_even(time){
            // use a slice to extract a subset of the possible values
            // take the first half of the possible_valuesay
            possible_values = possible_values[0..middle_index].to_vec();
        }
        else {
            // take the 2nd half of the possible_valuesay
            possible_values = possible_values[middle_index..len].to_vec();
        }
        let sl = (time%3) + seed;
        sleep(Duration::from_millis(sl));
    }
    return dice_roll
}
