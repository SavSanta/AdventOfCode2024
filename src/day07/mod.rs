use utils::fast_parse_int;
use super::*;

pub struct Day;


fn equation_valid(expectedresult: u64, result: u64, index: u64, nums: Vec<u64> ) -> bool {
    // Recursive function as seen in others in Java, Kotlin, and CSharp
    // HyperNeutrino has a great more math-y but concise solver. 
    // Some the from highlighted backtracking and/or DFS to solve.
    // Someone else really clever mentioned using the properties of maths such as divisibility and others. To optomize.
    // 5  = 2 + 2 + 1
    if result > expectedresult {
        return false;
    }

    // may need to change to >= in case my indexing somehow jumps more than by one lol.
    if index == nums.len() as u64 {
        // If we're at the last index of the array then we have no more numbers
        // May need to minus by 1 depending on 0-base or not contributes a factor

        // Since we're on the last loop we can check the overall result 
        if  result == expectedresult {
            return true; 
        }
        return false
    }

    let current_num = nums[index as usize];
    let new_result = result + current_num;
    if new_result <= expectedresult {
        if equation_valid(expectedresult, new_result, index+1, nums.clone()) {
            return true;
        }
    }

    let new_result = result * current_num;
    if new_result <= expectedresult {
        return equation_valid(expectedresult, new_result, index+1, nums.clone());
    }

    return false;
}

fn equation_valid2(expectedresult: u64, result: u64, index: u64, nums: Vec<u64> ) -> bool {
    // Part 2 we have an additonal operation check which involves concatenation. This function adds it as a third test

    if result > expectedresult {
        return false;
    }

    if index >= nums.len() as u64 {
        // If we're at the last index of the array then we have no more numbers
        // May need to minus by 1 depending on 0-base or not contributes a factor

        // Since we're on the last loop we can check the overall result 
        if  result == expectedresult {
            return true; 
        }
        return false
    }


    let current_num = nums[index as usize];

    let new_result = result + current_num;
    if new_result <= expectedresult {
        if equation_valid2(expectedresult, new_result, index+1, nums.clone()) {
            return true;
        }
    }

    let new_result = result * current_num;
    if new_result <= expectedresult {
        if equation_valid2(expectedresult, new_result, index+1, nums.clone()) {
            return true;
        }
    }
    
    //let prev_num: u64 = nums[(&index - 1) as usize];  <--- Failure here.
    // There is a failure as well on using newresult as the var. The function must use the result as below
    // Need to study the recursiveness of this understand how recursive shit mentally works.
    // Study studt. Study!
    let concat_num : u64 = (result.to_string() + &current_num.to_string()).parse().expect("Issues with concatentation");
    if concat_num <= expectedresult
    {
        let is_correct = equation_valid2(expectedresult, concat_num, &index+1, nums.clone());
        //println!(" Correctness test of prev_num {prev_num}, current_num {current_num} = concat_num {concat_num} ==> {is_correct}");
        return is_correct;
    }

    return false;
}

impl SolutionSilver<u64> for Day {
    const DAY: u32 = 7;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> u64 {
        
        #[derive(Debug)]
        struct Equation {
            total : u128,
            numbers : Vec<u64>,
        }
      
        // Organize the data into 
        let mut parsed_data: Vec<Equation> = Vec::new();
        parsed_data = input
            .lines()
            .map(|line| {
                let (total, nums) = line.split_once(": ").expect("Things went wrong on processing");
                let numbers = nums.split(" ").map(|num| num.parse::<u64>().expect("Err: Processing u64s")).collect();
                Equation { total: total.parse::<u128>().unwrap(), numbers }
            }).collect();
    
       let solu1= parsed_data.iter()
                                        .filter(|x| equation_valid(x.total as u64, 0, 0, x.numbers.clone()))
                                        .collect::<Vec<_>>()
                                        .into_iter()
                                        .fold(0, |acc, x| acc + x.total);

    solu1 as u64
    }

}

impl SolutionGold<u64, u64> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> u64 {
        #[derive(Debug)]
        struct Equation {
            total : u128,
            numbers : Vec<u64>,
        }
      
        // Organize the data into 
        let mut parsed_data: Vec<Equation> = Vec::new();
        parsed_data = input
            .lines()
            .map(|line| {
                let (total, nums) = line.split_once(": ").expect("Things went wrong on processing");
                let numbers = nums.split(" ").map(|num| num.parse::<u64>().expect("Err: Processing u64s")).collect();
                Equation { total: total.parse::<u128>().unwrap(), numbers }
            }).collect();
    
       let solu1= parsed_data.iter()
                                        .filter(|x| equation_valid2(x.total as u64, 0, 0, x.numbers.clone()))
                                        .collect::<Vec<_>>()
                                        .into_iter()
                                        .fold(0, |acc, x| acc + x.total);

        solu1 as u64
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(3749, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(7885693428401, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(11387, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(348360680516005, output);
}
