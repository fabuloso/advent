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
      acc += dividable(line.unwrap());
      return acc
    })
}


fn dividable(numbers: String) -> u32 {
    let mut parts:Vec<u32> = numbers
    .split_whitespace()
    .map(|s| s.parse::<u32>().unwrap()).collect();

    let (first, second) = asd(parts.pop().unwrap(), parts);

    return first / second
}   

fn asd(first: u32, mut lst:Vec<u32>) -> (u32, u32) {
    for second in lst.iter() {
        if (first % second) == 0 { return (first, *second) }
        if (second % first) == 0 { return (*second, first) }
    }
    return asd(lst.pop().unwrap() , lst)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn given_a_string_with_two_number_returns_the_sum_of_them() {
        assert_eq!(2, dividable(String::from("5   10")))
    }

    #[test]
    fn given_a_string_with_two_number_returns_thess_sum_of_them() {
        assert_eq!(2, dividable(String::from("31   5   10")))
    }

    #[test] 
    fn it_sums_only_the_max_and_the_min_on_the_row() {
        assert_eq!(5, dividable(String::from("15 7 3")))
    }

}