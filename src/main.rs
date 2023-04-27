use itertools::Itertools;
use std::collections::{BTreeSet};

const COLS: [&str; 4] = ["blue", "black", "red", "yellow"];

fn main() {

    let ball_orders = COLS.into_iter().permutations(4).to_owned()
                        .filter(|perm| perm.into_iter().position(|&col| col == "blue").unwrap() < perm.into_iter().position(|&col| col == "black").unwrap())
                        .collect::<Vec<Vec<&str>>>();
    
    let mut adj_pairs = BTreeSet::<[Vec<&str>; 2]>::new();

    for ball_order in ball_orders.iter() {
        adj_pairs.extend(ball_swap(ball_order.to_owned()));
    }
    println!("Neighbour pairs {:?}", adj_pairs);
    


}

fn ball_swap(fr: Vec<&str>) -> BTreeSet::<[Vec<&str>; 2]> {

    let mut adj_pair = BTreeSet::<[Vec<&str>; 2]>::new();

    for (id, ball) in fr.iter().enumerate() {
        for (idx, swap) in fr[id + 1..].iter().enumerate() {
            if ball == &"blue" && swap == &"black"  {continue}
            let mut new = fr.clone();
            new[id] = swap;
            new[id + idx + 1] = ball;
            adj_pair.insert([fr.clone(), new.clone()]);
        }
    }

    adj_pair
}
