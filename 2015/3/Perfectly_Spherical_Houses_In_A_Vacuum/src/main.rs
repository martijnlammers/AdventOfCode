use std::fs;
use std::collections::HashSet;
fn main() 
{

    let mut directions = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut santa_direction = String::from("");
    let mut robo_santa_direction = String::from("");
    
    for (index, direction) in directions.chars().into_iter().enumerate()
    {
        if index % 2 == 0
        {
            santa_direction.push(direction);
        }
        else 
        {
            robo_santa_direction.push(direction);
        }
    }
    let santa_nodes: Vec<Node> = create_nodes(&santa_direction);
    let robo_santa_nodes: Vec<Node> = create_nodes(&robo_santa_direction);
    let mut nodes: Vec<Node> = Vec::new();
    nodes.extend(santa_nodes);
    nodes.extend(robo_santa_nodes);
    let hash_set: HashSet<Node> = nodes.into_iter().collect();
    println!("Deliveries: {}", hash_set.len());

}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Node 
{
    x: i16,
    y: i16,
}

fn delivered_presents(directions: &String) -> i16
{
    let nodes: Vec<Node> = create_nodes(&directions);
    let hash_set: HashSet<Node> = nodes.into_iter().collect();
    hash_set.len() as i16
}

fn create_nodes(directions: &String) -> Vec<Node>
{
    let mut current_node: Node = Node{x:0,y:0};
    let mut nodes: Vec<Node> = Vec::new();
    nodes.push(current_node.clone());

    for direction in directions.chars()
    {
        match direction
        {
            '>' => current_node.x += 1,
            '<' => current_node.x -= 1,
            '^' => current_node.y += 1,
            'v' => current_node.y -= 1,
            _ => panic!("unrecognised character")
        }
        nodes.push(current_node.clone());
    }
    nodes
}


#[test]
fn delivered_presents_test()
{
    let directions: String = String::from(">");
    assert_eq!(delivered_presents(&directions), 2);

    let directions: String = String::from("^>v<");
    assert_eq!(delivered_presents(&directions), 4);

    let directions: String = String::from("^v^v^v^v^v");
    assert_eq!(delivered_presents(&directions), 2);
}