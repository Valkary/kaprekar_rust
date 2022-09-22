use std::io::stdin;

fn main() {
    println!("Welcome to the Kaprekar Constant Calculator (with steps!)");

    println!("Input the number you want to test: ");

    let mut input_num = String::new();
   
    stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");

    let num: u32 = input_num.trim().parse().expect("Input not an integer");

    calculate_kaprekar(num);

}

fn fill_with_zeroes(num: u32) -> String {
    let mut num_string = num.to_string();
    let zeroes_to_fill = 4 - num_string.len();
   
    for _ in 0..zeroes_to_fill {
        num_string = format!("0{num_string}");
    }

    num_string
}

fn order_numbers(num: u32) -> (u32, u32) {
    let mut digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let zeroes_vec = vec![0; 4 - digits.len()];

    digits.extend_from_slice(&zeroes_vec);

    digits.sort();

    let mut highest_to_lowest: String = "".to_owned();
    let mut lowest_to_highest: String = "".to_owned();

    for digit in &digits {
        highest_to_lowest = format!("{}{}", digit, highest_to_lowest);
        lowest_to_highest = format!("{}{}", lowest_to_highest, digit);
    }

    let big_num: u32 = highest_to_lowest.parse().unwrap();
    let small_num: u32 = lowest_to_highest.parse().unwrap();

    (big_num, small_num)
}

fn calculate_kaprekar(num: u32) {
    let mut result = num;
    let mut iter: u32 = 0;

    'calculate: loop {
        iter += 1;
        let ordered_nums = order_numbers(result);
        let (larger, smaller) = ordered_nums;
        result = larger - smaller;

        println!("iter: {} | {} | {} | {}", iter, fill_with_zeroes(larger), fill_with_zeroes(smaller), fill_with_zeroes(result));

        if result == 6174 || result == 0 { 
            break 'calculate;
        } else {
            continue 'calculate;
        }
    }
}
