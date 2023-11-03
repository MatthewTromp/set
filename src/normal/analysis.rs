use std::{collections::{HashSet, HashMap}, thread::{spawn, JoinHandle}, sync::mpsc::sync_channel, ops::Deref};

use super::backend::{Game, make_shuffled_deck, make_deck, Card, third_card};

const NUM_THREADS: usize = 16;

pub fn print_num_cards_to_forced_set(count: usize) {
    let map = (0..count).map(|_| {
        let v = most_cards_with_no_set();
       // println!("{}", v);
        v})
        .fold([0; 21], |mut m, v| {
            m[v] += 1;
            m
        });

    for (num, occurences) in map.iter().enumerate() {
        if occurences != &0 {
            println!("{}: {} ({}%)", num, occurences, 100.*(*occurences as f64)/(count as f64));
        }
    }
}

pub fn print_num_cards_to_forced_set_multithreaded(count: usize) {
    // thread to keep track of the counts
    let mut arr = [0; 21];
    let (sender, receiver) = sync_channel(100);
    let _ = spawn(move || {
        loop {
            match receiver.recv() {
                Err(_) => break,
                Ok(v) => arr[v] += 1
            }
        }
    });
    println!("Starting threads");
    let handles: Vec<JoinHandle<()>> = (0..NUM_THREADS).map(|_| {
        let sender = sender.clone();
        spawn(move || {
            (0..(count/NUM_THREADS)).for_each(|_| {
                sender.send(most_cards_with_no_set()).unwrap();
            });
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }

    for (num, occurences) in arr.iter().enumerate() {
        if occurences != &0 {
            println!("{}: {} ({}%)", num, occurences, 100.*(*occurences as f64)/(count as f64));
        }
    }
}
    
        

pub fn most_cards_with_no_set() -> usize {
    let mut cards: HashSet<Card> = HashSet::new();
    for c in make_shuffled_deck() {
        let mut bad_card = false;
        for c2 in cards.iter() {
            if cards.contains(&third_card(&c, c2)) {
                bad_card = true;
                break;
            }
        }
        if bad_card {
            continue;
        }
        cards.insert(c);
    }
    cards.len()
}

pub fn most_cards_with_no_set_fast() -> usize {
    let mut cards: HashSet<Card> = HashSet::new();
    for c in make_shuffled_deck() {
        let mut bad_card = false;
        for c2 in cards.iter() {
            if cards.contains(&third_card(&c, c2)) {
                bad_card = true;
                break;
            }
        }
        if bad_card {
            continue;
        }
        cards.insert(c);
    }
    cards.len()
}
