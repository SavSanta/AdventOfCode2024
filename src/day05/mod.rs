use utils::fast_parse_int;
use super::*;
use std::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::sync::{LazyLock,Mutex};

// Previous compiler suggestion but wouldnt work as mutable. Have to move to Mutex below
//static rules: LazyLock<HashMap<u128, Vec<u128>>> = LazyLock::new(|| { HashMap::new()});
//static rules: Mutex<HashMap<u128, Vec<u128>>> = Mutex::new(HashMap::new().insert(7777 , vec![7777]));
static mut updates: Vec<Vec<u128>> = Vec::new();


// Failed above with statics/global so trying redefine of Day
pub struct Day { rules: HashMap<u128, Vec<u128>>}

trait FakeBubbleSort {
    unsafe fn fake_bubble_sort(&self, a : u128, b: u128) -> std::cmp::Ordering;



}
/*
impl FakeBubbleSort for Day {
    
    unsafe fn fake_bubble_sort(&self, a: u128, b: u128) -> std::cmp::Ordering {

    // Would be cleaner as a match but need to research how to pullout the vec. Prob another nested match
        let mut map = HashMap::new();
    map.insert(128, vec![144,177,128,761]);
    let test = map.get_key_value(&128);
    match test {
            Some((u128, _)) => { println!("Think it matched {:?}", test.unwrap()[1]); },
            None => { println!("No Match Baby");},
            _ => { panic!("Should not have reached unmatchable"); }
   
    } 

    if !self.rules.contains_key(&a) {
        return std::cmp::Ordering::Equal;
    }


    if self.rules[&a].contains(&b) {
        return std::cmp::Ordering::Greater;
    }
    else {
        return std::cmp::Ordering::Less;
    }
 

}*/

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 5;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    
    fn calculate_silver(input: &str) -> usize {
/*        let mut total = 0;
        const BUFFER_LEN: usize = 99 - 11 + 1;
    
    unsafe {
        let (rules_data, updates_data) = input.split_once("\n\n").unwrap();
    
            /* Failure to get this to work as a pipeline so.....
            rules_data.lines().map(|item| item.split_terminator('|'))
                                                                    
                                                                     .collect::<HashMap<u128,Vec<u128>>>();
                                                                    //.into_iter()
                                                                    //.map(|x| HashMap::from(x));
            */
    
            // So we go oldschool-ish with the help of GPT
            for line in rules_data.lines() {
            // Split the line by the pipe character '|', and collect the parts into a vector of strings
            let parts: Vec<&str> = line.split_terminator('|').collect();
            
            // Parse the first part as the key (u128), and the remaining parts as values (Vec<u128>)
            if let Some(&key_str) = parts.get(0) {
                // Parse the key (first element in the split)
                if let Ok(key) = key_str.parse::<u128>() {
                    // Parse the remaining parts as a vector of u128 values
                    let value: u128 = parts[1].parse::<u128>().ok().unwrap();

                    // This is a rough one due to mutability and RUST strictness.
                    // One has to perform a get_mut over insert here as HashMap doesnt implement IndMut 
                    if rules.contains_key(&key) 
                    {
                        rules.get_mut(&key).unwrap().push(value);
                    }
                    else {
                        // Need to find out the macro less version
                        rules.insert(key, vec![value]);
                    }

                }
            }
        }

        // Begin populating updates 
        updates = updates_data.lines().map(|line| {
            line.split_terminator(",")
                .map(|m| m.parse::<u128>().unwrap()) 
            }
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    }
        
        


        return  total;
**/
    return 2 as usize;
    }

} 

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
/*         
        (rules, updates) = input.split_once("\n\n").unwrap();
        rules: Vec<(usize, usize)> = rules
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('|').unwrap();
                (fast_parse_int(l), fast_parse_int(r))
            })
            .collect();
        let updates: Vec<Vec<usize>> = updates
            .lines()
            .map(|line| line.split(',').map(fast_parse_int).collect())
            .collect();
        
        updates
            .into_iter()
            .filter(|u| {
                // inverted
                !rules.iter().all(|r| {
                    match (
                        u.iter().position(|&x| x == r.0),
                        u.iter().position(|&x| x == r.1),
                    ) {
                        (Some(p0), Some(p1)) => p0 <= p1,
                        _ => true,
                    }
                })
            })
            .map(|mut u| {
                let mut correct = false;

                while !correct {
                    rules.iter().for_each(|r| {
                        let a = u.iter().position(|&x| x == r.0);
                        let b = u.iter().position(|&x| x == r.1);

                        if let (Some(a), Some(b)) = (a, b) {
                            if a > b {
                                u.swap(a, b);
                            }
                        }
                    });

                    correct = rules.iter().all(|r| {
                        match (
                            u.iter().position(|&x| x == r.0),
                            u.iter().position(|&x| x == r.1),
                        ) {
                            (Some(p0), Some(p1)) => p0 <= p1,
                            _ => true,
                        }
                    });
                }

                
            })
            .map(|u| u[u.len() / 2])
            .sum()**/
            return 2 as usize;
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(143, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(4185, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(123, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(4480, output);
}
