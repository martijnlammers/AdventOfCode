use std::fs::read_to_string;
fn main() {
    let content: Vec<String> = read_lines("input.txt");
    let mut wrapping_paper: i32 = 0;
    let mut ribbon: i32 = 0;

    for dimension in content
    {
        let input: Vec<i32> = format_input(&dimension);
        wrapping_paper += calculate_wrapping_paper(&input);
        ribbon += calculate_ribbon(&input);
    }

    println!("Required wrapping paper: {wrapping_paper} ft");
    println!("Required ribbon: {ribbon} ft");
    
}

fn calculate_ribbon(dimensions: &Vec<i32>) -> i32
{
    let mut ribbon: i32 = 0;
    ribbon += calculate_ribbon_box(dimensions);
    ribbon += calculate_ribbon_bow(dimensions);
    ribbon
}

fn calculate_ribbon_box(dimensions: &Vec<i32>) -> i32
{
    let mut dimensions: Vec<i32> = dimensions.to_vec();
    dimensions.sort();
    dimensions.pop();
    
    let length: i32 = dimensions[0];
    let width: i32 = dimensions[1];
    return 2 * (length + width);
}

fn calculate_ribbon_bow(dimensions: &Vec<i32>) -> i32
{
    let length: i32 = dimensions[0];
    let width: i32 = dimensions[1];
    let height: i32 = dimensions[2];
    return length * width * height;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn format_input(dimensions: &String) -> Vec<i32>
{
    let dimensions = dimensions.split("x");
    let dimensions: Vec<i32> = dimensions
        .map(|number: &str| number.parse::<i32>().unwrap()).collect();

    dimensions
}

fn calculate_wrapping_paper(dimensions: &Vec<i32>) -> i32
{
    let surface_area: i32 = calculate_box_surface_area(&dimensions);
    let slack: i32 = calculate_slack(&dimensions);
    let wrapping_paper: i32 = surface_area + slack;
    wrapping_paper
}

fn calculate_box_surface_area(dimensions: &Vec<i32>) -> i32
{
    
    const L: usize = 0;
    const W: usize  = 1;
    const H: usize  = 2;
    let length: i32 = dimensions[L];
    let width: i32 = dimensions[W];
    let height: i32 = dimensions[H];
    let surface_area: i32 = 2*((length*width) + (width*height) + (height*length));
    surface_area
}

fn calculate_slack(dimensions: &Vec<i32>) -> i32
{
    let mut dimensions: Vec<i32> = dimensions.to_vec();
    dimensions.sort();
    dimensions.pop();
    let slack: i32 = dimensions[0] * dimensions[1];
    slack
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_input_test()
    {
        let input: String = String::from("2x3x4");
        assert_eq!(format_input(&input), Vec::from([2,3,4]));

        let input: String = String::from("1x1x10");
        assert_eq!(format_input(&input), Vec::from([1,1,10]));
    }

    #[test]
    fn calculate_wrapping_paper_test()
    {
        let input: Vec<i32> = Vec::from([1,1,10]);
        assert_eq!(calculate_wrapping_paper(&input), 43);

        let input: Vec<i32> = Vec::from([2,3,4]);
        assert_eq!(calculate_wrapping_paper(&input), 58);
    }

    #[test]
    fn calculate_box_surface_area_test()
    {
        let input: Vec<i32> = Vec::from([2,3,4]);
        assert_eq!(calculate_box_surface_area(&input), 52);

        let input: Vec<i32> = Vec::from([1,1,10]);
        assert_eq!(calculate_box_surface_area(&input), 42);
    }

    #[test]
    fn calculate_slack_test()
    {
        let input: Vec<i32> = Vec::from([2,3,4]);
        assert_eq!(calculate_slack(&input), 6);

        let input: Vec<i32> = Vec::from([1,1,10]);
        assert_eq!(calculate_slack(&input), 1);
    }

    #[test]
    fn calculate_ribbon_test()
    {
        let input: Vec<i32> = Vec::from([2,3,4]);
        assert_eq!(calculate_ribbon(&input), 34);

        let input: Vec<i32> = Vec::from([1,1,10]);
        assert_eq!(calculate_ribbon(&input), 14);
    }

    #[test]
    fn calculate_ribbon_box_test()
    {
        let input: Vec<i32> = Vec::from([2,3,4]);
        assert_eq!(calculate_ribbon_box(&input), 10);

        let input: Vec<i32> = Vec::from([1,1,10]);
        assert_eq!(calculate_ribbon_box(&input), 4);
    }

    #[test]
    fn calculate_ribbon_bow_test()
    {
        let input: Vec<i32> = Vec::from([2,3,4]);
        assert_eq!(calculate_ribbon_bow(&input), 24);

        let input: Vec<i32> = Vec::from([1,1,10]);
        assert_eq!(calculate_ribbon_bow(&input), 10);
    }
}