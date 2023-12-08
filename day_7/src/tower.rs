use regex::Regex;
use core::fmt;
use std::{io::{self, BufRead}, collections::HashMap, borrow::BorrowMut};
use std::borrow::Cow;

#[derive(Eq)]
#[derive(Debug)]

struct Hand<'a> {
    cards: Cow<'a, str>,
    bid: i32,
    score_1: i32,
    score_2: i32,
    score_3: i32,
    score_4: i32,
    score_5: i32,
    score_6: i32
}

impl <'a>PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl <'a>Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score_1 != other.score_1 {
            return self.score_1.cmp(&other.score_1);
        }
        if self.score_2 != other.score_2 {
            return self.score_2.cmp(&other.score_2);
        }
        if self.score_3 != other.score_3 {
            return self.score_3.cmp(&other.score_3);
        }
        if self.score_4 != other.score_4 {
            return self.score_4.cmp(&other.score_4);
        }
        if self.score_5 != other.score_5 {
            return self.score_5.cmp(&other.score_5);
        }
        self.score_6.cmp(&other.score_6)
    }
}

impl <'a>fmt::Display for Hand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bid {}, Cards {}", self.bid, self.cards)
    }    
}

impl <'a>PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
            && self.score_1 == other.score_1
            && self.score_2 == other.score_2
            && self.score_3 == other.score_3
            && self.score_4 == other.score_4
            && self.score_5 == other.score_5
            && self.score_6 == other.score_6
    }
}

fn count_cards(cards: &str) -> Option<(Vec<&i32>, char)> {
    let mut no_cards: HashMap<char, i32> = HashMap::new();
    for c in cards.chars() {    // Count cards
        if !no_cards.contains_key(&c) {
            print!("1st {c}");
            no_cards.insert(c, 1);
        } else {
            print!("+ {c}");
            no_cards.insert(c, no_cards.get(&c).unwrap() + 1);
        }
    }
    let card_values = no_cards.clone();
    let mut values: Vec<&i32> = no_cards.values().collect();
    values.sort();
    values.reverse();
    let mut most_common: char = 'z';
    for (key, val) in no_cards {
        if (val == *values[0]) {
            most_common = key;
        }
    }
    if most_common == 'z' {
        return None;
    }
    return Some((values, most_common));
}

fn get_hand_type(cards: &str) -> i32 {
    let (_, most_common) = count_cards(cards).unwrap();
    // Sort out Js
    let improved_cards = cards.replace("J", most_common.to_string().as_str());

    let (values, _) = count_cards(&improved_cards).unwrap();
    if values.len() == 1 {
        return 7;   // Five of a kind
    }
    print!("{:?}", values);
    if **values.get(0).unwrap() == 4 {
        return 6; // 4 of a kind 
    }
    if **values.get(0).unwrap() == 3 && **values.get(1).unwrap() == 2 {
        return 5;  // full house
    }
    if **values.get(0).unwrap() == 3 {
        return 4; // 3 of a kind 
    }
    if **values.get(0).unwrap() == 2 && **values.get(1).unwrap() == 2 {
        return 3;  // 2 pair
    }
    if **values.get(0).unwrap() == 2 {
        return 2; // 1 pair
    }
    return 1;
}

fn get_card_val(card: char) -> i32 {
    return match card.to_string().parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            match card {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                _ => 14 // ace
            }
        },
    }
}

fn main() {
    let mut handler = io::stdin().lock();
    let mut eof = false;
    let mut line = String::new();
    let re = Regex::new(r"([\d\w]+)\s+(\d+)").unwrap();
    let mut hands: Vec<Hand> = Vec::new();
    while !eof {
        match handler.read_line(&mut line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                let caps = re.captures(line.as_str()).unwrap();
                let cards = caps.get(1).unwrap().as_str().trim().clone();
                let bid: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let score_1 = get_hand_type(cards);
                let mut chars = cards.chars();
                let score_2 = get_card_val(chars.next().unwrap());
                let score_3 = get_card_val(chars.next().unwrap());
                let score_4 = get_card_val(chars.next().unwrap());
                let score_5 = get_card_val(chars.next().unwrap());
                let score_6 = get_card_val(chars.next().unwrap());
                hands.push(Hand {
                    cards: Cow::Owned(cards.to_string()),
                    bid,
                    score_1,
                    score_2,
                    score_3,
                    score_4,
                    score_5,
                    score_6
                });
                line.clear();
            }
            Err(_error) => {
                panic!("Problem !1");
            }
        }
    }
    // Sort hands
    let mut winnings = 0;
    hands.sort();
    for i in 0..hands.len() {
        let rank = (hands.len() - i) as i32;
        let score = hands[i].bid * rank;
        get_hand_type(&hands[i].cards);
        println!("Rank {} {} Score: {} {:?}", rank, hands[i], score, hands[i]);
        // println!("{:?}", hands[i]);
        winnings += score;
    }
    println!("{winnings}");
}
