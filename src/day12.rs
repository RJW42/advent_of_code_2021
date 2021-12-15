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


    // Finished 
    Ok(())
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
        let n1_id = nodes[n2].id;
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