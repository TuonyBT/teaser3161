use itertools::Itertools;
use std::collections::{BTreeSet, BTreeMap};

const COLS: [&str; 4] = ["blue", "black", "red", "yellow"];

fn main() {

    let mut valid_paths = Vec::<Vec<&Vec<&str>>>::new();

    let ball_orders = COLS.into_iter().permutations(4).to_owned()
                        .filter(|perm| perm.into_iter().position(|&col| col == "blue").unwrap() < perm.into_iter().position(|&col| col == "black").unwrap())
                        .collect::<Vec<Vec<&str>>>();
    
    let mut adj_pairs = BTreeMap::<&Vec<&str>, BTreeSet<Vec<&str>>>::new();
    let mut nadj_pairs = BTreeMap::<&Vec<&str>, BTreeSet<Vec<&str>>>::new();

    for ball_order in ball_orders.iter() {
        adj_pairs.insert(ball_order, ball_swap(ball_order.to_owned()).0);
        nadj_pairs.insert(ball_order, ball_swap(ball_order.to_owned()).1);

    }

    for node in ball_orders.iter() {
//        println!("Node {:?}", node);
//        println!("Adjacent neighbour pairs {:?}", adj_pairs[node]);
//        println!("Non-adjacent neighbour pairs {:?}", nadj_pairs[node]);
//        println!();

        let mut new_edges = vec![vec![node]];
        for pass in 1..13 {

            let mut full_paths = Vec::<Vec<&Vec<&str>>>::new();
            for start in &new_edges {
//                println!("Start {:?}", start.last().unwrap());
    
                let nbrs = match pass %5 {
                    0 => &nadj_pairs[start.last().unwrap()],
                    _ => &adj_pairs[start.last().unwrap()],
                };
                full_paths.extend(edges(start.to_owned(), nbrs));
            }
            new_edges = full_paths.to_owned();
            
            if pass == 11 && full_paths.len() > 0 {    
//                println!("Node {:?}", node);
//                println!("New edges found on pass {}: {:?}", pass, full_paths);
//                println!("New edges found on pass {}: {:?}", pass, full_paths.iter().map(|z| z.len()).collect::<Vec<usize>>());
//                println!();
                valid_paths.extend(full_paths);
            }
        }

    }
    for p in valid_paths {
        for col in COLS {
            let before_yellow = p.iter()
            .map(|z| z.iter().position(|c| c == &col) < z.iter().position(|c| c == &"yellow"))
            .collect::<Vec<bool>>();

            if before_yellow[0] == false {continue}
            
            let crossovers = before_yellow[1..].into_iter().enumerate()
            .map(|(idx, &val)| before_yellow[idx] == val)
            .filter(|&b| b == false)
            .count();
            if crossovers == 1 {
                println!("Unique sequence matching solution {:?}", p);
                println!();
                println!("Colour referred to in puzzle is {}. Last hoop order {:?}", col, p.last().unwrap());
            }
        }

    }
}

fn edges<'a>(seq: Vec<&'a Vec<&str>>, nbrs: &'a BTreeSet<Vec<&str>>) -> Vec<Vec<&'a Vec<&'a str>>> {

    let mut new_paths = Vec::<Vec<&Vec<&str>>>::new();
    let avail_nbrs = nbrs.iter().filter(|z| !seq.contains(z));

    for nbr in avail_nbrs {
        let mut new_path = seq.to_owned();
        new_path.push(nbr);
        new_paths.push(new_path);
    }

    new_paths
}

fn ball_swap(fr: Vec<&str>) -> (BTreeSet<Vec<&str>>, BTreeSet<Vec<&str>>) {

    let mut adj_pair = BTreeSet::<Vec<&str>>::new();
    let mut nadj_pair = BTreeSet::<Vec<&str>>::new();

    for (id, ball) in fr.iter().enumerate() {
        for (idx, swap) in fr[id + 1..].iter().enumerate() {
//            if ball == &"blue" && swap == &"black"  {continue}
            if ball == &"blue" && idx >= fr[id + 1..].iter().position(|z| z == &"black").unwrap()  {continue}
            if swap == &"black" && id < fr.iter().position(|z| z == &"blue").unwrap()  {continue}
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
