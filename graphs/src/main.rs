use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct GraphErr {
    mess: String,
}

impl GraphErr {
    fn new(s: &str) -> Self {
        GraphErr {
            mess: s.to_string(),
        }
    }
}

// Mappointer based
#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, dt: T) {
        // node has no edges yet
        self.data.insert(id, (dt, Vec::new()));
    }

    pub fn add_edge(
        &mut self,
        edge_id: ID,
        from: ID,
        to: ID,
        edge_data: E,
    ) -> Result<(), GraphErr> {
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new("'from' not in nodes"));
        }
        if let Some(ref mut dt) = self.data.get_mut(&to) {
            self.edges
                .insert(edge_id.clone(), (edge_data, from.clone(), to));
            dt.1.push(edge_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }

        self.data.get_mut(&from).unwrap().1.push(edge_id);
        Ok(())
    }
}

fn main() -> Result<(), GraphErr> {
    let mut g = Graph::new();

    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, ());
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'G', 'G', 5)?;
    g.add_edge('g', 'H', 'A', 8)?;
    g.add_edge('h', 'H', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;

    println!("Hello, graph {:?}", g);
    Ok(())
}
