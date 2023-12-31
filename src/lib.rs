// I don't think I like these data structures

struct Atom(String);

enum Value {
    Values(Vec<Atom>),
    Pair(Box<Pair>),
}

struct Pair {
    key: String,
    value: Box<Value>,
}

struct Laml {
    expressions: Vec<Pair>,
}

pub fn laml2yaml(laml: &str) -> String {
    // parse laml into datastructures
    // convert to yaml
    todo!()
}
