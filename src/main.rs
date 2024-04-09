use crate::simple_find::{naive_string_matcher, Printer};

pub mod simple_find;

//
// enum VertexState {
//     Link,
//     Nil
// }
//
// struct Vertex {
//     from: Vec<Box<Vertex>>,
//     state: VertexState
// }
//
// impl Vertex {
//     pub fn new() -> Self {
//         Self { from: vec![], state: VertexState::Nil }
//     }
//
//     pub fn from() {}
//
//     pub fn from_is_empty(&self) -> bool {
//         self.from.is_empty()
//     }
//
//     pub fn add(&mut self, input: Box<Vertex>) {
//         self.from.push(input);
//         self.state = VertexState::Link;
//     }
// }
//
// struct AhoCorasik {
//
// }

fn main() {
    let t = naive_string_matcher("new lol kek".to_string(), "lol k".to_string());
    println!("{}", Printer(t));
}
