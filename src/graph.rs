extern mod core;

use core::hash::Hash;
use core::hashmap::{HashMap, HashSet};

// #[crate_type = "lib"];

trait Graph<Node, Edge> {
    fn children(&self, &Node, &fn(&(Node, Edge)) -> bool);   // iterate on outgoing edges
    fn add_node(&mut self, Node);
    fn node_count(&self) -> uint;
    fn add_edge(&mut self, Node, Node, Edge);
    fn each_node(&self, &fn(&Node) -> bool);
}

fn edge_count<N,E, G:Graph<N,E>>(g: &G) -> int {
    let mut count = 0;
    each_edge(g, |_,_,_| { count += 1; true });
    count
}

fn each_edge<N,E, G:Graph<N,E>>(g: &G, f: &fn(&N,&N,&E) -> bool){
    g.each_node(|source| {
        g.children(source, |&(target, edge)| { f(source, &target, &edge) });
        true
    })
}

fn dfs<N:Hash+Eq+Copy,E, G:Graph<N,E>>(g: &G, source: &N, visitor: &fn(&N) -> bool) {
    let mut visited = ~HashSet::new();
    visited.insert(*source);
    dfs_rec(g, source, visitor, visited);
}

fn dfs_rec<N:Hash+Eq+Copy,E, G:Graph<N,E>>(g: &G, source: &N, visitor: &fn(&N) -> bool, visited: &mut HashSet<N>) {
    if visitor(source){
        g.children(source, |&(target,_)| {
            if !visited.contains(&target) {
                visited.insert(target);
                dfs_rec(g, &target, visitor, visited);
            };
            true
            });
    }
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

    fn each_node(&self, f: &fn(&N) -> bool) {
        self.each_key(|source| {f(source)});
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
    let mut g = HashMap::new();
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
    dfs(&g, &0, |&n| {nodes.push(n); true });

    if nodes.len() != 3 {
        fail!("Wrong number of elements visited");
    }
    if nodes[0] != 0 || nodes[1] != 1 || nodes[2] != 2 {
        fail!("Wrong elements");
    }
}
