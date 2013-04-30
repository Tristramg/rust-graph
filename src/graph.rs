extern mod std;

trait Graph<Node, Edge> {
    fn children(&self, &fn(&(Edge, Node)));   // iterate on outgoing edges
}

