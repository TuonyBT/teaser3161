use itertools::Itertools;

const COLS: [&str; 4] = ["blue", "black", "red", "yellow"];

fn main() {

    let ball_orders = COLS.into_iter().permutations(4).to_owned()
                        .filter(|perm| perm.into_iter().position(|&col| col == "blue").unwrap() < perm.into_iter().position(|&col| col == "black").unwrap())
                        .collect::<Vec<Vec<&str>>>();
    
    for ball_order in ball_orders.iter() {
        println!("Ball order {:?}", ball_order);
        println!("Ball swaps {:?}", ball_swap(ball_order.to_owned()));
        println!();
    }
    


}

fn ball_swap(fr: Vec<&str>) -> Vec<Vec<&str>> {

    let mut out = Vec::<Vec<&str>>::new();

    for (id, ball) in fr.iter().enumerate() {
        for (idx, swap) in fr[id + 1..].iter().enumerate() {
            if ball == &"blue" && swap == &"black"  {continue}
            let mut new = fr.clone();
            new[id] = swap;
            new[id + idx + 1] = ball;
            out.push(new);

        }
    }

    out
}
