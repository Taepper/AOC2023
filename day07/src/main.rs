use std::cmp::{max, min, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input9.txt";

    let file = File::open(filename)?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut hands : Vec<(u32, u32)> = Vec::new();

    for line_result in lines{
        let line = line_result.unwrap();
        let line_parts: Vec<&str>  = line.split_whitespace().collect();
        assert_eq!(line_parts.len(), 2);
        let hand = line_parts[0];
        let bid : u32 = line_parts[1].parse::<u32>().unwrap();

        let mut hand_value : u32 = 0;
        let mut count_per_card = [0u32; 13];
        for card in hand.chars(){
            let cards = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
            let card_value = cards.iter().position(|&r| r == card).unwrap();
            count_per_card[card_value ] += 1;
            hand_value <<= 4;
            hand_value += 12 - (card_value as u32);
        }
        count_per_card.sort();
        hand_value += count_per_card[11] << 20;
        hand_value += count_per_card[12] << 24;

        hands.push((hand_value, bid));
    }
    hands.sort();

    let mut sum = 0;

    for (index, (hand, bid)) in hands.iter().enumerate() {
        sum += bid * (index as u32 + 1u32);
        let type_value1 = hand >> 24;
        let type_value2 = (hand >> 20) & 0xF ;
        println!("Rank {}: {},{} type_value: ({}, {})", index + 1, hand, bid, type_value1, type_value2);
    }

    println!("Result {}", sum);

    return Ok(());
}
