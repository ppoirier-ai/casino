use rand::Rng;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

const PAYOUT_WIN_SCENARIOS: [&str; 6] = ["JJJ", "777", "333", "222", "111", "CCC"];
const PAYOUT_MULTIPLIER: [i32; 6] = [1199, 0200, 0100, 0090, 0040, 0040];

const REEL: [char; 64] = ['J', '7', '3', '2', '1', 'C', '2', '1', 'C', '7', '3', '1', 'J', '2', '1', '3', '1', 'C', '7', '2', '1', '1', 'J', '3', '2', '1', 'C', '7', '1', '3', '2', '1', 'C', 'J', '7', '2', '1', '3', '1', '1', 'C', '7', '2', '1', 'J', '3', '1', '2', '1', 'C', '7', '3', '1', '2', '1', 'J', '7', '3', '1', 'C', '2', '1', '1', '1'];
const REEL_SIZE: usize = REEL.len() - 1;

pub fn spin() ->  (i32, String) {
    let reel1 = rand::thread_rng().gen_range(0..=REEL_SIZE);
    let reel2 = rand::thread_rng().gen_range(0..=REEL_SIZE);
    let reel3 = rand::thread_rng().gen_range(0..=REEL_SIZE);
    let spinout = format!("{}{}{}", REEL[reel1], REEL[reel2], REEL[reel3]);
    println!("what's the latest symbol in the reel {}", spinout);

    // calculate the payout
    match spinout.as_str() {
    //    _ if spinout == PAYOUT_WIN_SCENARIOS[0] => println!("You won the {}!!! {}", PAYOUT_WIN_SCENARIOS[0], PAYOUT_MULTIPLIER[0]),
        _ if spinout == PAYOUT_WIN_SCENARIOS[0] => return (PAYOUT_MULTIPLIER[0], PAYOUT_WIN_SCENARIOS[0].to_string()),
        _ if spinout == PAYOUT_WIN_SCENARIOS[1] => return (PAYOUT_MULTIPLIER[1], PAYOUT_WIN_SCENARIOS[1].to_string()),
        _ if spinout == PAYOUT_WIN_SCENARIOS[2] => return (PAYOUT_MULTIPLIER[2], PAYOUT_WIN_SCENARIOS[2].to_string()),
        _ if spinout == PAYOUT_WIN_SCENARIOS[3] => return (PAYOUT_MULTIPLIER[3], PAYOUT_WIN_SCENARIOS[3].to_string()),
        _ if spinout == PAYOUT_WIN_SCENARIOS[4] => return (PAYOUT_MULTIPLIER[4], PAYOUT_WIN_SCENARIOS[4].to_string()),
        _ if spinout == PAYOUT_WIN_SCENARIOS[5] => return (PAYOUT_MULTIPLIER[5], PAYOUT_WIN_SCENARIOS[5].to_string()),
        _ => return (0, spinout),
    }
}


