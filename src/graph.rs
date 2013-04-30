extern mod core;

use core::hashmap::linear::LinearMap;
use core::hashmap::linear::linear_map_with_capacity;

// #[crate_type = "lib"];

trait Graph<Node, Edge> {
    fn children(&self, Node, &fn(&(Edge, Node)) -> bool);   // iterate on outgoing edges
}

impl<E> Graph<int,E> for ~LinearMap<int, ~[(E,int)]> {
    fn children(&self, n:int, f : &fn(&(E,int)) -> bool) {
        match self.find(&n) {
            None => (),
            Some(list) => list.each(f)
        };
    }
}

#[test]
fn test_1() {
    let g : ~LinearMap<int, int> = ~linear_map_with_capacity(15);
    // let g2 : Graph<int,int> = ~*g;
}
