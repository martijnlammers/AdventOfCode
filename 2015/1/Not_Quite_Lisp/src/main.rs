use std::fs;

fn main() 
{
    let mut directions = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    println!("Floor number: {}", calculate_floor(&mut directions));
}

fn calculate_floor(directions: &mut String) -> i16
{
    let mut floor: i16 = 0;
    for (index, direction) in directions.chars().into_iter().enumerate()
    {
        if floor == -1
        {
            println!("Reached basement at index: {}", index);
        }

        if direction == '('
        {
            floor += 1;
        }
        else 
        {
            floor -= 1;
        }
    }
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
