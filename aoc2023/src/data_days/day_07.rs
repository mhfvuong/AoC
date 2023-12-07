use std::collections::HashMap;
use std::hash::Hash;

pub fn day_07(data_string: String){
    let lines = data_string.lines();
    let mut hands = Vec::new();
    let mut bids = Vec::new();
    let mut hands_rank = HashMap::new();
    let mut hands_type = HashMap::new();

    for line in lines{
        let hand_bid = line.split(" ").collect::<Vec<&str>>();
        hands.push(hand_bid[0]);
        bids.push(hand_bid[1].parse::<i32>().unwrap());
        hands_rank.insert(hand_bid[0], 1);
        let mut cards = HashMap::new();
        for card in hand_bid[0].chars() {
            if let Some(x) = cards.get_mut(&card) {
                *x += 1;
            }
            else {cards.insert(card, 1);}
        }
        // println!("hand: {}", &hand_bid[0]);
        // println!("{:?}", &cards);
        let mut three_of_a_kind = false;
        let mut two_of_a_kind = false;
        for card in cards.keys(){ // determine what the hand is
            if let Some(x) = cards.get(card) {
                // println!("The card: {}, appeared: {}", &card, &x);
                match x {
                    5  => { hands_type.insert(hand_bid[0], "Five of a kind"); },
                    4 => { hands_type.insert(hand_bid[0], "Four of a kind"); },
                    3 => {
                        if two_of_a_kind {
                            hands_type.insert(hand_bid[0], "Full house");
                        } else {
                            hands_type.insert(hand_bid[0], "Three of a kind");
                            three_of_a_kind = true;
                        }
                    },
                    2 => {
                        if three_of_a_kind {
                            hands_type.insert(hand_bid[0], "Full house");
                        } else if two_of_a_kind {
                            hands_type.insert(hand_bid[0], "Two pair");
                        } else {
                            hands_type.insert(hand_bid[0], "One pair");
                            two_of_a_kind = true;
                        }
                    },
                    _ => ()
                }
            }
            if hands_type.get(hand_bid[0]) == None {
                hands_type.insert(hand_bid[0], "High card");
            }
        }
        println!("hand: {}", &hand_bid[0]);
        println!("{:?}", &hands_type.get(hand_bid[0]));
        let mut joker = false;
        for card in cards.keys() {
            if !joker {
                if *card == 'J' {
                    change_hand_type(&cards, card, &mut hands_type, hand_bid[0]);
                    joker = true;
                    println!("changes to {:?}", &hands_type.get(hand_bid[0]));
                }
            }
        }
    }
    let mut hands_checked = 0;
    let mut type_idx = 0;
    let mut type_to_be_checked  = ["High card", "One pair", "Two pair", "Three of a kind", "Full house", "Four of a kind", "Five of a kind"];
    while hands_checked < hands.len() {
        let mut hands_to_be_checked = Vec::new();
        for hand in &hands {
            if let Some(c_type) = hands_type.get(hand) {
               if c_type == &type_to_be_checked[type_idx] {
                   hands_to_be_checked.push(*hand);
               }
            }
        }
        for hand in &hands_to_be_checked{ // update to min rank
            if let Some(rank) = hands_rank.get_mut(hand){
                *rank += hands_checked;
            }
        }
        if !hands_to_be_checked.is_empty() {
            hands_checked += 1;
        }
        if hands_to_be_checked.len() == 1 || hands_to_be_checked.is_empty(){
            type_idx += 1;

            continue;
        }
        let mut n = 0;
        // println!("there are {} cards with {}", &hands_to_be_checked.len(), &type_to_be_checked[type_idx]);
        while n < hands_to_be_checked.len() { // compare each hand with one another

            let hand_a = hands_to_be_checked[n];
            for hand_b in &hands_to_be_checked {
                if hand_a != *hand_b {
                    compare(hand_a, *hand_b, &mut hands_rank);
                }
            }
            n += 1;
        }
        hands_checked += hands_to_be_checked.len() - 1;
        type_idx += 1;
    }
    println!("{:?}", hands_rank);

    let mut winnings = 0;
    let mut ranking = Vec::new();
    for i in 0..hands.len() {
        let hand = hands[i];
        let bid = bids[i];
        if let Some(rank) = hands_rank.get_mut(hand) {
            ranking.push(rank.clone());
            winnings += bid * *rank as i32;
        }
    }
    println!("all winnings {}", winnings);
}

fn compare(hand_a: &str, hand_b: &str, hands_rank: &mut HashMap<&str, usize> ) {
    let mut rank_determined = false;
    for i in 0..5 {
        if !rank_determined {
            let mut a = 1;
            let mut b = 1;
            match hand_a.chars().nth(i){
                Some('A') => a = 14,
                Some('K') => a = 13,
                Some('Q') => a = 12,
                Some('J') => a = 1,
                Some('T') => a = 10,
                Some(x) => a = x.to_string().parse::<i32>().unwrap(),
                None => (),
            }
            match hand_b.chars().nth(i){
                Some('A') => b = 14,
                Some('K') => b = 13,
                Some('Q') => b = 12,
                Some('J') => b = 1,
                Some('T') => b = 10,
                Some(x) => b = x.to_string().parse::<i32>().unwrap(),
                None => (),
            }

            if a > b {
                if let Some(rank) = hands_rank.get_mut(hand_a){
                    *rank += 1;
                    rank_determined = true;
                }
            } else if a < b {
                break;
            }
        }
    }
}
fn change_hand_type(cards: &HashMap<char, i32>, card: &char, hands_type: &mut HashMap<&str, &str>, hand: &str) {
    match cards.get(card) { // find the amount of jokers
        Some(1) => {if let Some(hand_type) = hands_type.get_mut(hand) {
            if *hand_type == "Four of a kind" { *hand_type = "Five of a kind"; }
            else if *hand_type == "Three of a kind" { *hand_type = "Four of a kind"; }
            else if *hand_type == "Two pair" { *hand_type = "Full house"; }
            else if *hand_type == "One pair" { *hand_type = "Three of a kind"; }
            else if *hand_type == "High card" { *hand_type = "One pair"; }
        }},
        Some(2) => {if let Some(hand_type) = hands_type.get_mut(hand) {
            if *hand_type == "Full house" {
                *hand_type = "Five of a kind";}
            else if *hand_type == "Two pair" {*hand_type = "Four of a kind";}
            else if *hand_type == "One pair" {*hand_type = "Three of a kind";}
        }},
        Some(3) => {if let Some(hand_type) = hands_type.get_mut(hand) {
            if *hand_type == "Full house" {*hand_type = "Five of a kind";}
            else if *hand_type == "Three of a kind" {*hand_type = "Four of a kind";}
        }},
        Some(4) => {if let Some(hand_type) = hands_type.get_mut(hand) {
            *hand_type = "Five of a kind";
        }},
        _ => (),
    }
}