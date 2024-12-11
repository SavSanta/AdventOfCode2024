use utils::fast_parse_int;
use super::*;

pub struct Day;

// 5  = 2 + 2 + 1
fn equation_valid(expectedresult: u64, result: u64, index: u64, nums: Vec<u64> ) -> bool {
    // Recursive function as seen in others in Java, Kotlin, and CSharp
    // HyperNeutrino has a great more mathy but concise solver

    if result > expectedresult {
        return false;
    }

    // may need to change to >= in case my indexing somehow jumps more than by one lol.
    if index == nums.len() as u64 {
        // If we're at the last index of the array then we have no more numbers
        // May need to minus by 1 depending on 0-base or not contributes a factor

        // CSince we're on the last loop we vcan check the overall result 
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
        input
            .lines()
            .map(|line| {
                let (out, rest) = line.split_once(':').unwrap();
                let expected_result = fast_parse_int(out) as u64;
                let rest = rest
                    .trim()
                    .split(" ")
                    .map(fast_parse_int)
                    .map(|a| a as u64)
                    .collect::<Vec<_>>();

                for i in 0..(3usize.pow(rest.len() as u32)) {
                    let mut i = i;
                    let mut current_result = 0u64;
                    for &num in &rest {
                        // if number is already too large, we won't get the expected result with future operations
                        if current_result >= expected_result {
                            break;
                        }

                        let operation = i % 3;
                        i /= 3;

                        match operation {
                            0 => current_result *= num,
                            1 => current_result += num,
                            // assume that the first number is never 0, as the exercise does not define it
                            2 if current_result == 0 => break,
                            2 => {
                                current_result = 10u64.pow(num.ilog10() + 1) * current_result + num;
                            }
                            _ => unreachable!(),
                        }
                    }

                    if current_result == expected_result {
                        return expected_result;
                    }
                }
                0
            })
            .sum()
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
    assert_eq!(70597497486371, output);
}
