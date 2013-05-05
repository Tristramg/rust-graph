extern mod core;

use core::hash::Hash;
use core::hashmap::HashMap;
use core::hashmap::linear_map_with_capacity;

// #[crate_type = "lib"];

trait Graph<Node, Edge> {
    fn children(&self, &Node, &fn(&(Node, Edge)) -> bool);   // iterate on outgoing edges
    fn add_node(&mut self, Node);
    fn node_count(&self) -> uint;
    fn add_edge(&mut self, Node, Node, Edge);
    //    fn each_node(&self, &fn(&Node) -> bool);
    fn each_edge(&self, &fn(&Node, &Node, &Edge) -> bool);

    //fn edge_count(&self) -> uint;
}

fn edge_count<N,E, G:Graph<N,E>>(g: &G) -> int {
    let mut count = 0;
    g.each_edge(|_,_,_| {
            count += 1;
            true
        });
    count
}

impl<N:Hash+Eq,E> Graph<N,E> for HashMap<N, ~[(N,E)]> {
    fn children(&self, n:&N, f : &fn(&(N,E)) -> bool) {
        match self.find(n) {
            None => (),
            Some(list) => list.each(f)
        };
    }

    fn add_node(&mut self, n:N) {
        self.insert(n,~[]);
    }

    fn node_count(&self) -> uint {
        self.len()
    }

    fn add_edge(&mut self, source:N, target:N, e:E) {
        match self.find_mut(&source) {
            None => fail!("Source node does not exist"),
            Some(list) => list.push((target, e))
        }
    }

    fn each_edge(&self, f: &fn(&N,&N,&E) -> bool){
        self.each_key(|&source| {
            self.children(&source, |&(target, edge)| {
                f(&source, &target, &edge);
                true
            });
            true
        })
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

#[test]
fn add_node_string() {
    let mut g : HashMap<~str, ~[(~str,int)]> = linear_map_with_capacity(15);
    if g.node_count() != 0 {
        fail!("The graph is not empty");
    }

    g.add_node(~"Hii");
    if g.node_count() != 1 {
        fail!("Node not added");
    }
}

#[test]
fn add_edge() {
    let mut g : HashMap<int, ~[(int,int)]> = linear_map_with_capacity(15);
    if edge_count(&g) != 0 {
        fail!("The graph is not empty");
    }

    g.add_node(0);
    g.add_node(1);
    g.add_edge(0,1,1);

    if edge_count(&g) != 1 {
        fail!("Not one edge");
    }
}

#[test]
fn add_edge_string() {
    let mut g : HashMap<~str, ~[(~str,int)]> = linear_map_with_capacity(15);
    if edge_count(&g) != 0 {
        fail!("The graph is not empty");
    }

    g.add_node(~"a");
    g.add_node(~"b");
    g.add_edge(~"a",~"b",1);

    if edge_count(&g) != 1 {
        fail!("Not one edge");
    }
}
