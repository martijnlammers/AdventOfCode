use std::fs::read_to_string;

#[derive(Clone, Debug, Copy)]
struct Point2D
{
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Copy)]
struct Cell
{
    coordinates: Point2D,
    lights: bool,
    brightness: i32,
}
#[derive(Debug)]
enum Mode
{
    Toggle,
    Off,
    On,
    IncreaseTwice,
    Increase,
    Lower,
}


fn main() {

    let grid:&mut Vec<Cell> = &mut make_grid(1000, 1000);

    for line in read_to_string("input.txt").unwrap().lines() {
        println!("Lights turned on: {:?}", part_one(grid, &line.to_string()));
        println!("Brightness level: {:?}", part_two(grid, &line.to_string()));
    }
}

fn make_grid(width: i32, height: i32) ->  Vec<Cell>
{
    let total_cells: usize = usize::try_from(width * height).unwrap();
    let mut grid: Vec<Cell> = vec![Cell{coordinates: Point2D{x:0, y:0}, lights:false, brightness: 0}; total_cells];
    let mut current = Point2D {x: 0, y:0};
    grid.iter_mut().for_each(
        |cell: &mut Cell|
        {
            cell.coordinates = current;

            current.x += 1;

            if current.x % width == 0
            {
                current.x = current.x % width;
                current.y += 1;
            }
        }
    );

    grid
}

fn part_one(grid: &mut Vec<Cell>, instruction: &String) -> i32
{
    let instruction: Vec<&str> = instruction.split_whitespace().collect();

    let action = instruction[0];
    let mode = instruction[1];
    match action
    {
        "turn" => {
            match mode
            {
                "on" => turn_on(grid, &instruction),
                "off" => turn_off(grid, &instruction),
                _ => return 0
            }
        }
        "toggle" => toggle(grid,&instruction),
        _ => return 0
    }

    let lights_count = grid.iter().filter(|cell| cell.lights).count() as i32; 
    
    lights_count
}

fn part_two(grid: &mut Vec<Cell>, instruction: &String) -> i32
{
    let instruction: Vec<&str> = instruction.split_whitespace().collect();

    let action = instruction[0];
    let mode = instruction[1];
    match action
    {
        "turn" => {
            match mode
            {
                "on" => increase_brightness(grid, &instruction),
                "off" => lower_brightness(grid, &instruction),
                _ => return 0
            }
        }
        "toggle" => increase_brightness_twice(grid,&instruction),
        _ => return 0
    }

    let mut brightness: i32 = 0;

    grid.iter().for_each(|cell: &Cell| brightness += cell.brightness);
    
    brightness
}

fn increase_brightness_twice(grid: &mut Vec<Cell>, tokens: &Vec<&str>)
{
    let (top_left, bottom_right) = parse_coordinates(&tokens, 1, 3);
    modify_lights(grid, top_left, bottom_right, Mode::IncreaseTwice);
}

fn lower_brightness(grid: &mut Vec<Cell>, tokens: &Vec<&str>)
{
    let (top_left, bottom_right) = parse_coordinates(&tokens, 2, 4);
    modify_lights(grid, top_left, bottom_right, Mode::Lower);
}

fn increase_brightness(grid: &mut Vec<Cell>, tokens: &Vec<&str>)
{
    let (top_left, bottom_right) = parse_coordinates(&tokens, 2, 4);
    modify_lights(grid, top_left, bottom_right, Mode::Increase);
}

fn parse_coordinates(tokens: &Vec<&str>, tl_index: usize, br_index: usize) -> (Point2D, Point2D)
{
    let coordinates:Vec<&str> = tokens[tl_index].split(",").collect();

    let top_left = Point2D{
        x:coordinates[0].parse().unwrap(),
        y:coordinates[1].parse().unwrap()};

    let coordinates:Vec<&str> = tokens[br_index].split(",").collect();
    let bottom_right = Point2D{
        x:coordinates[0].parse().unwrap(),
        y:coordinates[1].parse().unwrap()};
    
    return (top_left, bottom_right)
}

fn toggle(grid: &mut Vec<Cell>, tokens: &Vec<&str>)
{
    let (top_left, bottom_right) = parse_coordinates(&tokens, 1, 3);
    modify_lights(grid, top_left, bottom_right, Mode::Toggle);
}

fn turn_on(grid: &mut Vec<Cell>, tokens: &Vec<&str>)
{
    let (top_left, bottom_right) = parse_coordinates(&tokens, 2, 4);
    modify_lights(grid, top_left, bottom_right, Mode::On);
}

fn turn_off(grid: &mut Vec<Cell>, tokens: &Vec<&str>)
{
    let (top_left, bottom_right) = parse_coordinates(&tokens, 2, 4);
    modify_lights(grid, top_left, bottom_right, Mode::Off);
}

fn modify_lights(grid: &mut Vec<Cell>, top_left: Point2D, bottom_right: Point2D, modification: Mode)
{
    for row in top_left.y .. bottom_right.y + 1
    {
        for col in top_left.x .. bottom_right.x + 1
        {
            let pointer: usize = (col + (row * 1000)).try_into().unwrap();
            
            match modification
            {
                Mode::Toggle => grid[pointer].lights = !grid[pointer].lights,
                Mode::On => grid[pointer].lights = true,
                Mode::Off => grid[pointer].lights = false,
                Mode::Increase => grid[pointer].brightness += 1,
                Mode::IncreaseTwice => grid[pointer].brightness += 2,
                Mode::Lower => grid[pointer].brightness = if grid[pointer].brightness == 0 {0} else { grid[pointer].brightness - 1 },
            }
        }
    }
}



#[test]
fn test_make_grid()
{
    let grid = &mut  make_grid(1000,1000);
    assert_eq!(grid[0].coordinates.x, 0);
    assert_eq!(grid[0].coordinates.y, 0);
    assert_eq!(grid[999999].coordinates.x, 999);
    assert_eq!(grid[999999].coordinates.y, 999);
}

#[test]
fn test_part_one()
{
    let grid = &mut make_grid(1000, 1000);
    let instruction = String::from("turn on 0,0 through 999,999");
    assert_eq!(part_one(grid, &instruction), 1000000);
}


#[test]
fn test_part_two()
{
    let grid = &mut make_grid(1000, 1000);
    let instruction = String::from("turn on 0,0 through 0,0");
    assert_eq!(part_two(grid, &instruction), 1);

    let grid = &mut make_grid(1000, 1000);
    let instruction = String::from("toggle 0,0 through 999,999");
    assert_eq!(part_two(grid, &instruction), 2000000);

}
