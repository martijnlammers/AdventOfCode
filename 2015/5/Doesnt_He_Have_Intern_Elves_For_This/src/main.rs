use std::fs::read_to_string;
use fancy_regex::Regex;
fn main() {

    let mut nice_strings_count = 0;

    for string in read_to_string("input.txt").unwrap().lines() {

        if is_nice_pt2(&string.to_string())
        {
            nice_strings_count += 1;
        }
    }

    println!("{nice_strings_count}");
}

fn is_nice(input: &String) -> bool
{
    let three_vowels: Regex = Regex::new(r"(.*[aeiou]){3}").unwrap();
    let consecutive_chars: Regex = Regex::new(r"(.)\1").unwrap();
    let matching_pattern: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    let mut result: bool = is_match(&three_vowels, &input);
    if !result
    {
        return false;
    }

    result = is_match(&consecutive_chars, &input);
    if !result
    {
        return false;
    }

    result = is_match(&matching_pattern, &input);
    if !result
    {
        return true;
    }
    false
}

fn is_nice_pt2(input: &String) -> bool
{
    // Make a set of characters (..)
    // Make a set of the whole string .*
    // Look if group 1 is in the string \1
    
    let condition1: Regex = Regex::new(r"(..).*\1").unwrap();

    // Take any character and put in group 1.
    // Look for any other character
    // Look for another character that has been placed in group 1.

    let condition2: Regex = Regex::new(r"(.)(.)\1").unwrap();

    let mut result: bool = is_match(&condition1, &input);
    if !result
    {
        return false;
    }

    result = is_match(&condition2, &input);
    if !result
    {
        return false;
    }

    true
}

fn is_match(regex: &Regex, input: &String) -> bool
{
    let result = regex.captures(input);

    match result 
    {
        Err(_err) => return false,
        Ok(_res) => 
        {
            match _res 
            {
                None => return false,
                Some(_s) => 
                {
                    return true
                },
                
            }
        }
    }
}

#[test]
fn test_is_nice()
{
    let input = String::from("ugknbfddgicrmopn");
    assert_eq!(is_nice(&input), true);

    let input = String::from("aaa");
    assert_eq!(is_nice(&input), true);

    let input = String::from("jchzalrnumimnmhp");
    assert_eq!(is_nice(&input), false);

    let input = String::from("haegwjzuvuyypxyu");
    assert_eq!(is_nice(&input), false);

    let input = String::from("dvszwmarrgswjxmb");
    assert_eq!(is_nice(&input), false);
}

#[test]
fn test_is_nice_pt2()
{
    let input = String::from("qjhvhtzxzqqjkmpb");
    assert_eq!(is_nice_pt2(&input), true);

    let input = String::from("xxyxx");
    assert_eq!(is_nice_pt2(&input), true);

    let input = String::from("uurcxstgmygtbstg");
    assert_eq!(is_nice_pt2(&input), false);

    let input = String::from("ieodomkazucvgmuy");
    assert_eq!(is_nice_pt2(&input), false);

}