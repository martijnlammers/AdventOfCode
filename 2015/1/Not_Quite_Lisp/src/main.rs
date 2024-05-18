use std::fs;

fn main() 
{
    let mut directions = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    println!("Floor number: {}", calculate_floor(&mut directions));
}

fn calculate_floor(input: &mut String) -> i16
{
    let total: i16 = input.len() as i16;
    input.retain(|c| c == '(');

    let up: i16 = input.len() as i16;
    let down: i16 = total - up;
    let floor: i16 = up - down;

    println!("Floors up: {} \nFloors down: {} \nFloor: {}", up, down, floor);
    return floor;
}

#[cfg(test)]
mod unit_tests 
{
    use crate::calculate_floor;

    #[test]
    fn calculate_floor_test() 
    {
        let mut input = String::from("(())");
        assert_eq!(calculate_floor(&mut input), 0);

        input = String::from("(()(()(");
        assert_eq!(calculate_floor(&mut input), 3);

        input = String::from("))(((((");
        assert_eq!(calculate_floor(&mut input), 3);

        input = String::from(")())())");
        assert_eq!(calculate_floor(&mut input), -3);
    }
}
