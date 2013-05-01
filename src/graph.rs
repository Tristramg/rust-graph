extern mod core;

use core::hashmap::HashMap;
use core::hashmap::linear_map_with_capacity;

// #[crate_type = "lib"];

trait Graph<Node, Edge> {
    fn children(&self, Node, &fn(&(Edge, Node)) -> bool);   // iterate on outgoing edges
    fn add_node(&mut self, Node);
    fn node_count(&self) -> uint;
//    fn add_edge(&self, Edge);
//    fn edge_count(&self) -> int;
//    fn each_node(&self, &fn(&Node) -> bool);
//    fn each_edge(&self, &fn(&Edge) -> bool);
}

impl<E> Graph<int,E> for HashMap<int, ~[(E,int)]> {
    fn children(&self, n:int, f : &fn(&(E,int)) -> bool) {
        match self.find(&n) {
            None => (),
            Some(list) => list.each(f)
        };
    }

    fn add_node(&mut self, n:int) {
        self.insert(n,~[]);
    }

    fn node_count(&self) -> uint {
        self.len()
    }
}

#[test]
fn add_node() {
    let mut g : ~HashMap<int, ~[(int,int)]> = ~linear_map_with_capacity(15);
    if g.node_count() != 0 {
        fail!("The graph is not empty");
    }

    g.add_node(1);
    if g.node_count() != 1 {
        fail!("Node not added");
    }
}
