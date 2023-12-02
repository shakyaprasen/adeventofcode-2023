use std::{collections::HashMap, fs};

fn main() {
    let mut limit: HashMap<&str, i32> = HashMap::new();

    limit.insert("red", 12);
    limit.insert("green", 13);
    limit.insert("blue", 14);

    let mut total = 0;
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            println!("Next Split");
            let (_, game) = line.split_at(4);
            let Some((_, rest)) = game.split_once(':') else {
                panic!("Error")
            };
            let mut max: HashMap<&str, i32> = HashMap::new();
            for pull in rest.split(';') {
                let options: Vec<&str> = pull.split(',').collect();
                for option in options.iter() {
                    let Some((occ, color_type)) = option.trim().split_once(' ') else {
                        panic!("Error.")
                    };

                    let Ok(occ) = occ.trim().parse::<i32>() else {
                        panic!("erads")
                    };
                    let current_max = max.get(color_type);
                    let current_max = match current_max {
                        Some(val) => *val,
                        _ =>  0
                    };
                    if occ > current_max {
                        println!("{} - {}, {}", color_type, occ, current_max );
                        max.insert(color_type, occ);
                    }
                }
            }
            let mut product = 1;
            for val in &max {
                product *= val.1
            }
            product
        }).for_each(|product| {
            total += product;
        });

    println!("{}", total);
}
