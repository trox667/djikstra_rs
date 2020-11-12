use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Vertex {
    pub id: String,
    pub name: String,
}

impl Vertex {
    pub fn new(id: String, name: String) -> Self {
        Vertex { id, name }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Edge {
    pub id: String,
    pub source: Vertex,
    pub destination: Vertex,
    pub weight: i32,
}

impl Edge {
    pub fn new(id: String, source: Vertex, destination: Vertex, weight: i32) -> Self {
        Edge {
            id,
            source,
            destination,
            weight,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}
impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

#[derive(Debug)]
pub struct Djikstra {
    nodes: Vec<Vertex>,
    edges: Vec<Edge>,
    settled_nodes: HashSet<Vertex>,
    unsettled_nodes: HashSet<Vertex>,
    predecessors: HashMap<String, Vertex>,
    distance: HashMap<String, i32>,
}

impl Djikstra {
    pub fn new(graph: Graph) -> Self {
        Self {
            nodes: graph.vertices,
            edges: graph.edges,
            settled_nodes: HashSet::new(),
            unsettled_nodes: HashSet::new(),
            predecessors: HashMap::new(),
            distance: HashMap::new(),
        }
    }

    pub fn run(&mut self, source: &Vertex) {
        self.settled_nodes = HashSet::new();
        self.unsettled_nodes = HashSet::new();
        self.distance = HashMap::new();
        self.predecessors = HashMap::new();

        self.distance.insert(source.id.clone(), 0);
        self.unsettled_nodes.insert(source.clone());

        while self.unsettled_nodes.len() > 0 {
            if let Some(node) = self.get_minimum(&self.unsettled_nodes) {
                self.settled_nodes.insert(node.clone());
                self.unsettled_nodes.remove(&node);
                self.find_minimal_distance(&node);
            } else {
                panic!("Error");
            }
        }
    }

    fn find_minimal_distance(&mut self, node: &Vertex) {
        let adjacent_nodes = self.get_neighbors(node);
        for target in &adjacent_nodes {
            if self.get_shortest_distance(target)
                > self.get_shortest_distance(node) + self.get_distance(node, target)
            {
                self.distance.insert(
                    target.id.clone(),
                    self.get_shortest_distance(node) + self.get_distance(node, target),
                );
                self.predecessors.insert(target.id.clone(), node.clone());
                self.unsettled_nodes.insert(target.clone());
            }
        }
    }

    fn get_distance(&self, node: &Vertex, target: &Vertex) -> i32 {
        let mut weight = 0;
        for edge in &self.edges {
            if edge.source.id == node.id && edge.destination.id == target.id {
                weight = edge.weight;
            }
        }
        weight
    }

    fn get_neighbors(&self, node: &Vertex) -> Vec<Vertex> {
        let mut neighbors = vec![];
        for edge in &self.edges {
            if edge.source.id == node.id && !self.is_settled(&edge.destination) {
                neighbors.push(edge.clone().destination);
            }
        }
        neighbors
    }

    fn get_minimum(&self, vertices: &HashSet<Vertex>) -> Option<Vertex> {
        let mut minimum = None;
        for vertex in vertices {
            if minimum == None {
                minimum = Some(vertex.clone());
            } else if self.get_shortest_distance(vertex)
                < self.get_shortest_distance(&minimum.clone().unwrap())
            {
                minimum = Some(vertex.clone());
            }
        }
        return minimum;
    }

    fn is_settled(&self, vertex: &Vertex) -> bool {
        self.settled_nodes.contains(vertex)
    }

    fn get_shortest_distance(&self, destination: &Vertex) -> i32 {
        if let Some(d) = self.distance.get(&destination.id) {
            *d
        } else {
            std::i32::MAX
        }
    }

    pub fn get_path(&self, target: &Vertex) -> Vec<String> {
        let mut path = vec![];
        let mut step = target.clone();

        if self.predecessors.get(&step.id).is_none() {
            return vec![];
        } else {
            path.push(step.id.clone());
            while self.predecessors.get(&step.id.clone()).is_some() {
                if let Some(s) = self.predecessors.get(&step.id.clone()) {
                    step = s.clone();
                    path.push(step.id.clone());
                } else {
                    break;
                }
            }
        }

        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_lane(
        nodes: &Vec<Vertex>,
        edges: &mut Vec<Edge>,
        lane_id: String,
        source_loc_no: usize,
        dest_loc_no: usize,
        duration: i32,
    ) {
        let e = Edge::new(
            lane_id,
            nodes[source_loc_no].clone(),
            nodes[dest_loc_no].clone(),
            duration,
        );
        edges.push(e);
    }
    #[test]
    fn simple() {
        let mut nodes = vec![];
        let mut edges = vec![];

        nodes.push(Vertex::new("A".into(), "A".into()));
        nodes.push(Vertex::new("B".into(), "B".into()));
        nodes.push(Vertex::new("C".into(), "C".into()));
        nodes.push(Vertex::new("D".into(), "D".into()));
        nodes.push(Vertex::new("E".into(), "E".into()));
        add_lane(&nodes, &mut edges, "AB".into(), 0, 1, 10);
        add_lane(&nodes, &mut edges, "AD".into(), 0, 3, 80);
        add_lane(&nodes, &mut edges, "BE".into(), 1, 4, 20);
        add_lane(&nodes, &mut edges, "BC".into(), 1, 2, 50);
        add_lane(&nodes, &mut edges, "DC".into(), 3, 2, 50);
        add_lane(&nodes, &mut edges, "CE".into(), 2, 4, 50);
        add_lane(&nodes, &mut edges, "EC".into(), 4, 2, 20);
        add_lane(&nodes, &mut edges, "ED".into(), 4, 3, 40);

        let start = nodes[0].clone();
        let end = nodes[4].clone();
        let graph = Graph::new(nodes, edges);
        let mut djikstra = Djikstra::new(graph);
        djikstra.run(&start);
        let path = djikstra.get_path(&end);
        assert!(path.len() > 0);
        dbg!(path);
    }
    #[test]
    fn simple2() {
        let mut nodes = vec![];
        let mut edges = vec![];

        nodes.push(Vertex::new("A".into(), "A".into()));
        nodes.push(Vertex::new("B".into(), "B".into()));
        nodes.push(Vertex::new("C".into(), "C".into()));
        nodes.push(Vertex::new("D".into(), "D".into()));
        nodes.push(Vertex::new("E".into(), "E".into()));
        nodes.push(Vertex::new("F".into(), "F".into()));
        add_lane(&nodes, &mut edges, "AB".into(), 0, 1, 10);
        add_lane(&nodes, &mut edges, "AC".into(), 0, 2, 20);
        add_lane(&nodes, &mut edges, "BD".into(), 1, 3, 50);
        add_lane(&nodes, &mut edges, "BE".into(), 1, 4, 10);
        add_lane(&nodes, &mut edges, "CD".into(), 2, 3, 20);
        add_lane(&nodes, &mut edges, "CE".into(), 2, 4, 33);
        add_lane(&nodes, &mut edges, "DE".into(), 3, 4, 20);
        add_lane(&nodes, &mut edges, "DF".into(), 3, 5, 2);
        add_lane(&nodes, &mut edges, "EF".into(), 4, 5, 1);

        let start = nodes[0].clone();
        let end = nodes[5].clone();
        let graph = Graph::new(nodes, edges);
        let mut djikstra = Djikstra::new(graph);
        djikstra.run(&start);
        let path = djikstra.get_path(&end);
        assert!(path.len() > 0);
        dbg!(path);
    }
}
