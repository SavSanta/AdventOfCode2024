// This source failed on the real data but works on sample tes input. I've spent enough and ave given up. Learned a bit tho.


#[warn(unused_imports)]
use std::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;


// First implementation of a fake bubble sorting algorithim that i remembered from them Hungarians
// It didnt work in the main because the BS with static
// That being said it also was giving me the wrong order because apparently "Equals" doesnt need to be implemented. Either return Less or Greater
// See the final impl below to see what happened for directionality
/*fn fake_bubble_sort(a: u128, b: u128) -> std::cmp::Ordering {

    // Would be cleaner as a match but need to research how to pullout the vec. Prob another nested match
    /*    let mut map = HashMap::new();
    map.insert(128, vec![144,177,128,761]);
    let test = map.get_key_value(&128);
    match test {
            Some((u128, _)) => { println!("Think it matched {:?}", test.unwrap()[1]); },
            None => { println!("No Match Baby");},
            _ => { panic!("Should not have reached unmatchable"); }
    } */

    if !rules.contains_key(&a) {
        return std::cmp::Ordering::Equal;
    }

    if rules[&a].contains(&b) {
        return std::cmp::Ordering::Greater;
    }

    else {
        return std::cmp::Ordering::Less;
    }

}
*/

fn main() {
    let mut total = 0;
    const BUFFER_LEN: usize = 99 - 11 + 1;
    let mut rules: HashMap<u128, Vec<u128>> = HashMap::new();
    let mut updates: Vec<u128> = Vec::new();
    
    // Using sample data input
    //let (rules_data, updates_data) = input.split_once("\n\n").unwrap();

    // Using real data input
    use std::fs;
    let input2 = fs::read_to_string(r"C:\Users\crt\ProjectWork\AdventOfCode2024\src\day05\input_real.txt").expect("error reading file.");
    let (rules_data, updates_data) = input2.as_str().split_once("\n\n").unwrap();

    for line in rules_data.lines() {
        let parts: Vec<&str> = line.split_terminator('|').collect();
        if let Some(&key_str) = parts.get(0) {
            if let Ok(key) = key_str.parse::<u128>() {
                let value: u128 = parts[1].parse::<u128>().ok().unwrap();
                if rules.contains_key(&key) {
                    rules.get_mut(&key).unwrap().push(value);
                } else {
                    rules.insert(key, vec![value]);
                }
            }
        }
    }

        //********* Issue Below ***********/
    // Intake the file data to updates variables
    let mut updates = updates_data.lines().map(|line| {
        line.split_terminator(",")
            .map(|m| m.parse::<u128>().unwrap()) 
        }
        .collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let updates_cl = updates.clone();

    let test_closure = |x,y| -> std::cmp::Ordering {

        if !rules.contains_key(&x) {
            println!("It doesnt contain so return GREATer - {} {}", &x, &y);
            return std::cmp::Ordering::Greater;
        }
    
        if rules[&x].contains(&y) {
            println!("Confirm contained return LESS-er {} {}", &x, &y);
            return std::cmp::Ordering::Less;
        }

        else {
            println!("wE just do GREATer (BUT DUNNO WHY. it seems to work) {} {}", &x, &y);
            return std::cmp::Ordering::Greater;
        }
    
    };

    for update in &mut updates  {
        update.sort_by((|arg0: &u128, arg1: &u128| test_closure(*arg0, *arg1)));        
    }

    
    for (a, b) in updates.iter().zip(updates_cl.iter()) {
        if a.eq(b) {
            total += a[a.len() / 2];
        }
    }
    
    
    // /println!("###### RULESET ######\n{:?}", updates[0]);
    println!("Total is {}", total);
    //dbg!(&updates);
}

static input: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,4
"#;
