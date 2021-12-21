use std::fs::File;
use std::io::{BufReader, BufRead};


struct Graph {
    nodes: Vec<Node>,
    start_id: u32,
    end_id: u32,
}

struct Node {
    id: u32,
    name: String,
    adj: Vec<u32>,
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read Graph
    let graph = read_graph(file_name)?;

    // Print graph
    /*
    for n in &graph.nodes{
        print!("{}: ", n.name);
        for adj in &n.adj {
            print!("{} ", adj);
        }
        println!();
    } */

    // Init visited list
    let mut visited: Vec<bool> = Vec::new();

    for _i in 0..graph.nodes.len() {
        visited.push(false);
    }

    // Find all paths
    let result = find_all_paths(&graph, &mut visited, graph.start_id, graph.end_id, true);

    println!("Result: {}", result);


    // Finished
    Ok(())
}



fn find_all_paths(g: &Graph, v: &mut Vec<bool>, curr: u32, end: u32, can_double_v: bool) -> u32{
    // Base Case
    if v[curr as usize] && !can_double_v {
        return 0;
    }

    if v[curr as usize] && curr == g.start_id {
        return 0;
    }

    if curr == end {
        return 1;
    }

    // Mark current node as visited. Also, check if this is a double visited
    let mut is_double = false;

    if g.nodes[curr as usize].name.to_lowercase() == g.nodes[curr as usize].name {
        if v[curr as usize] {
            is_double = true;
        }

        v[curr as usize] = true;
    }

    // Loop through all adjacent nodes
    let mut result = 0;

    for nodes in &g.nodes[curr as usize].adj {
        result += find_all_paths(g, v, *nodes, end, can_double_v && !is_double);
    }

    // Unmark current node
    if !is_double {
        v[curr as usize] = false;
    }

    result
}



fn read_graph(file_name: String) -> std::io::Result<Graph> {
    // Read file
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    // Init vars
    let mut nodes = Vec::new();
    let mut start_id = 0;
    let mut end_id = 0;

    for line in br.lines(){
        // Get the current line
        let line = line?;

        // Get The two nodes
        let names = line.split('-').collect::<Vec<&str>>();

        // Get The nodes
        let n1 = get_node(&mut nodes, names[0]);
        let n2 = get_node(&mut nodes, names[1]);


        // Add and edge connecting these two nodes
        let n1_id = nodes[n1].id;
        let n2_id = nodes[n2].id;

        nodes[n1].adj.push(n2_id);
        nodes[n2].adj.push(n1_id);


        // Check if there are the start or end
        if nodes[n1].name == "start" {
            start_id = nodes[n1].id;
        } else if nodes[n1].name == "end" {
            end_id = nodes[n1].id;
        }

        if nodes[n2].name == "start" {
            start_id = nodes[n2].id;
        } else if nodes[n2].name == "end" {
            end_id = nodes[n2].id;
        }
    }

    // Finished
    Ok(Graph {
        nodes: nodes,
        start_id: start_id,
        end_id: end_id
    })
}


fn get_node(nodes: &mut Vec<Node>, name: &str) -> usize {
    // Check if node in nodes
    for i in 0..nodes.len() {
        let node = &nodes[i];

        if node.name == *name {
            return node.id as usize
        }
    }

    // Not, create new node and add to nodes
    let node = Node {
        id: nodes.len() as u32,
        name: name.to_string(),
        adj: Vec::new(),
    };

    nodes.push(node);

    nodes.len() - 1
}