// I don't htink I like this data structures
enum Value {
    Atom(String),
    Pair(Box<Pair>),
}

struct Pair {
    key: String,
    value: Box<Value>,
}

// struct Laml {
//     expressions: Vec<Expr>,
// }

pub fn laml2yaml(laml: &str) -> String {
    // parse laml into datastructure
    // convert to yaml
    todo!()
}
