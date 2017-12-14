use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
                
fn main() {   
    println!("{}", parse_file(String::from("input")))
}


fn parse_file(filename: String) -> u32 {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    
    file.lines().fold(0, |mut acc, line| {
        acc += parse(line.unwrap());
        return acc
    })

}

fn parse(numbers: String) -> u32 {
    let parts:Vec<u32> = numbers
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap()).collect();
    
    let maxvalue = parts.iter().max().unwrap();
    let minvalue = parts.iter().min().unwrap();

    println!("{} > {}", maxvalue, minvalue);
    maxvalue - minvalue
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn given_a_string_with_two_number_returns_the_sum_of_them() {
        assert_eq!(67, parse(String::from("15   52")))
    }

    #[test]
    fn it_sums_only_the_max_and_the_min_on_the_row() {
        assert_eq!(53, parse(String::from("15 1 52")))
    }

}