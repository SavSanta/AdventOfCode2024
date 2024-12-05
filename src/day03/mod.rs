use memchr::memmem;
use regex::bytes::Regex;
use utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 3;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        const MAX_NUM_LEN: usize = 4;

        use regex::Regex;
        use std::borrow::BorrowMut;               // in order to be able to borrow mutably without prelude

        let mut total: u32 = 0;

        // Using \d \w\ whatever tokens produces
        // rust Unicode-aware Perl class not found (make sure the unicode-perl feature is enabled)
        // Bug for more detail but ut's full of stuff: https://github.com/rust-lang/regex/issues/982
        let re_pattern = Regex::new(r"mul\([0-9]{1,3},[0-9]+\)").unwrap();
        let mut results = vec![];

        results = re_pattern.find_iter(input).map(|mut m| 
                                                            m.borrow_mut()
                                                            .as_str()
                                                            .replace("mul(", "")
                                                            .replace(")", ""))
                                                            .collect();

        // This is actually unused but this was as me trying to be more idiomatic (if not unreadable) and do it as a one-liner
        // Ask on StackO or Red about how to do this
        let splits = results.iter().map(|m| m.split_once(",").into_iter().collect::<Vec<_>>());

        // LLM teaching me how to split it out the long way with a bunch of lets. This makes up for above
        for item in results {
            if let Some((left, right)) = item.split_once(",")
                {
                    if let (Ok(left_into_num), Ok(right_into_num)) = (left.parse::<u32>(), right.parse::<u32>())
                        {
                            total += (left_into_num * right_into_num);
                        }
                }
        }
     
        // Multiplication table finished not return the values
        total as usize

        
        // In a future review visit go back above and edit reddit to use lookahead/around to skip the step where I replace the parentheses.


    } // calculate_silver
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        const MAX_NUM_LEN: usize = 4;

        use regex::Regex;
        use std::str::Matches;

        let mut total: u32 = 0;
        let mut enabled: bool = true;


        let re_pattern = Regex::new(r"mul\([0-9]{1,3},[0-9]+\)|don\'t\(\)|do\(\)").unwrap();
        let matches = re_pattern.find_iter(input);
        for matched  in matches {
            //println!(" Data was {:?}:" , matched);

            match matched.as_str() {
                
                "don't()" => enabled = false,
                "do()" => enabled = true,
                // My very very very very nasty and ugly hard to read implementation to derive the values. But it does work!!
                // Make this cleaner later. Maybe start with less parens
                _ => { 

                    println!("we was on some other time here {:?}",  matched.as_str()); 
                        if let Some((left, right)) = matched.as_str().replace("mul(", "").replace(")", "").split_once(",")
                        {
                            if let (Ok(left_into_num), Ok(right_into_num)) = (left.parse::<u32>(), right.parse::<u32>())
                                {
                                    if enabled {
                                        total += (left_into_num * right_into_num);
                                    }
                                
                                }
                        }
                    }
                    ,
            }
        }
        total as usize
    } //calculate gold
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(161, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(190604937, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(48, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(82857512, output);
}
