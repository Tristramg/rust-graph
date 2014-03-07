extern crate collections;

use std::hash::Hash;
use collections::{HashMap, HashSet};

trait Graph<Node, Edge> {
    // Mutation methods
    fn add_node(&mut self, Node);
    fn add_edge(&mut self, Node, Node, Edge);
    
    // Simple accessors
    fn node_count(&self) -> uint;

    // Structure iterators
    fn each_node(&self, |&Node|);
    fn children<'a>(&'a self, &Node, |&'a Node, &'a Edge|); // iterate on outgoing edges

    fn each_edge(&self, f: |&Node, &Node ,&Edge|){
        self.each_node(|source| {
            self.children(source, |target, edge| { f(source, target, edge) });
        })
    }

    fn edge_count(&self) -> uint {
        let mut count = 0;
        self.each_edge(|_,_,_| { count += 1; });
        count
    }

}

trait GraphAlgorithm<Node, Edge> {
    fn dfs_rec<'b>(&'b self, source: &'b Node, visitor: |&Node|, visited: &mut HashSet<&'b Node>);
}

impl<'a, Node:Hash+Eq, Edge> GraphAlgorithm<Node, Edge> for &'a Graph<Node, Edge> {
    fn dfs_rec<'b>(&'b self, source: &'b Node, visitor: |&Node|, visited: &mut HashSet<&'b Node>) {
        visitor(source);
        self.children(source, |target,_| {
            if !visited.contains(&source) {
            visited.insert(target);
            //        self.dfs_rec(target, visitor, visited);
            };
        });
    }
}





impl<N:Hash+Eq+Clone,E> Graph<N,E> for HashMap<N, ~[(N,E)]> {
    fn children<'a>(&'a self, out_node: &N, f : |&'a N, &'a E|) {
        match self.find(out_node) {
            None => (),
            Some(ref list) => for &(ref n, ref e) in list.iter() {
                f(n,e);
            }
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

    fn each_node(&self, f: |&N|) {
        for source in self.keys() {f(source)}
    }
}


#[test]
fn add_node() {
    let mut g : HashMap<int, ~[(int,int)]> = HashMap::new();
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
    let mut g : HashMap<~str, ~[(~str,int)]> = HashMap::new();
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
    let mut g = HashMap::new();
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

#[test]
fn add_edge_string() {
    let mut g = HashMap::new();
    println!("Count: {:?}", g.edge_count());
    if g.edge_count() != 0 {
        fail!("The graph is not empty");
    }

    g.add_node(~"a");
    g.add_node(~"b");
    g.add_edge(~"a",~"b",1);

    if g.edge_count() != 1 {
        fail!("Not one edge");
    }
}

/*
#[test]
fn dfs_test() {
    let mut g = HashMap::new();
    g.add_node(0);
    g.add_node(1);
    g.add_node(2);
    g.add_edge(0,1,1);
    g.add_edge(1,2,1);
    g.add_edge(2,0,1);

    let mut nodes = ~[];
    dfs(&g, 0, |n| {nodes.push(n); true });

    if nodes.len() != 3 {
        fail!("Wrong number of elements visited");
    }
    if nodes[0] != 0 || nodes[1] != 1 || nodes[2] != 2 {
        fail!("Wrong elements");
    }
}*/


