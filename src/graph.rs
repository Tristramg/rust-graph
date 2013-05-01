extern mod core;

use core::hashmap::HashMap;
use core::hashmap::linear_map_with_capacity;

// #[crate_type = "lib"];

trait Graph<Node, Edge> {
    fn children(&self, Node, &fn(&(Node, Edge)) -> bool);   // iterate on outgoing edges
    fn add_node(&mut self, Node);
    fn node_count(&self) -> uint;
    fn add_edge(&mut self, Node, Node, Edge);
    fn edge_count(&self) -> uint;
//    fn each_node(&self, &fn(&Node) -> bool);
    fn each_edge(&self, &fn(Node, Node, Edge) -> bool);
}

impl<E> Graph<int,E> for HashMap<int, ~[(int,E)]> {
    fn children(&self, n:int, f : &fn(&(int,E)) -> bool) {
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

    fn add_edge(&mut self, source:int, target:int, e:E) {
        match self.find_mut(&source) {
            None => fail!("Source node does not exist"),
            Some(list) => list.push((target, e))
        }
    }

    fn each_edge(&self, f: &fn(int,int,E) -> bool){
        self.each_key(|&source| {
            self.children(source, |&(target, edge)| {
                f(source, target, edge);
                true
            });
            true
        })
    }

    fn edge_count(&self) -> uint {
        let mut count = 0;
        self.each_edge(|_,_,_| {
            count += 1;
            true
        });
        count
    }
}

#[test]
fn add_node() {
    let mut g : HashMap<int, ~[(int,int)]> = linear_map_with_capacity(15);
    if g.node_count() != 0 {
        fail!("The graph is not empty");
    }

    g.add_node(1);
    if g.node_count() != 1 {
        fail!("Node not added");
    }
}

fn add_edge() {
    let mut g : HashMap<int, ~[(int,int)]> = linear_map_with_capacity(15);
    if g.edge_count() != 0 {
        fail!("The graph is not empty");
    }

    g.add_node(0);
    g.add_node(1);
    g.add_edge(0,1,1);

    if g.edge_count() != 1 {
        fail!("Not one edge");
    }
}
