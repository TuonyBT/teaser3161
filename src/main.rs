use itertools::Itertools;
use std::collections::{BTreeSet, BTreeMap};

const COLS: [&str; 4] = ["blue", "black", "red", "yellow"];

fn main() {

    let ball_orders = COLS.into_iter().permutations(4).to_owned()
                        .filter(|perm| perm.into_iter().position(|&col| col == "blue").unwrap() < perm.into_iter().position(|&col| col == "black").unwrap())
                        .collect::<Vec<Vec<&str>>>();
    
    let mut adj_pairs = BTreeMap::<&Vec<&str>, BTreeSet<Vec<&str>>>::new();
    let mut nadj_pairs = BTreeMap::<&Vec<&str>, BTreeSet<Vec<&str>>>::new();

    for ball_order in ball_orders.iter() {
        adj_pairs.insert(ball_order, ball_swap(ball_order.to_owned()).0);
        nadj_pairs.insert(ball_order, ball_swap(ball_order.to_owned()).0);

    }
    println!("Adjacent neighbour pairs {:?}", adj_pairs);
    println!("Non-adjacent neighbour pairs {:?}", nadj_pairs);
    


}

fn ball_swap(fr: Vec<&str>) -> (BTreeSet<Vec<&str>>, BTreeSet<Vec<&str>>) {

    let mut adj_pair = BTreeSet::<Vec<&str>>::new();
    let mut nadj_pair = BTreeSet::<Vec<&str>>::new();

    for (id, ball) in fr.iter().enumerate() {
        for (idx, swap) in fr[id + 1..].iter().enumerate() {
            if ball == &"blue" && swap == &"black"  {continue}
            let mut new = fr.clone();
            new[id] = swap;
            new[id + idx + 1] = ball;
            if idx == 0 {
                adj_pair.insert(new.clone());
            }
            else {
                nadj_pair.insert(new.clone());
            }
        }
    }

    (adj_pair, nadj_pair)
}
