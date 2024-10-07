use std::fmt;

struct Node {
    data: char,
}

struct Graph {
    nodes: Vec<Node>,
    matrix: Vec<Vec<i32>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            nodes: Vec::new(),
            matrix: vec![vec![0; size]; size],
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, src: usize, dst: usize) {
        self.matrix[src][dst] = 1;
    }

    fn check_edge(&self, src: usize, dst: usize) -> bool {
        self.matrix[src][dst] == 1
    }

    fn print(&self) {
        print!(" ");
        for node in &self.nodes {
            print!("{} ", node.data);
        }
        println!();

        for i in 0..self.matrix.len() {
            print!("{} ", self.nodes[i].data);
            for j in 0..self.matrix[i].len() {
                print!("{} ", self.matrix[i][j]);
            }
            println!();
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn main() {
    let mut graph = Graph::new(5);
    graph.add_node(Node { data: 'A' });
    graph.add_node(Node { data: 'B' });
    graph.add_node(Node { data: 'C' });
    graph.add_node(Node { data: 'D' });
    graph.add_node(Node { data: 'E' });

    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(1, 4);
    graph.add_edge(2, 3);
    graph.add_edge(2, 4);
    graph.add_edge(4, 0);
    graph.add_edge(4, 2);

    graph.print();
}