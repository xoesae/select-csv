use std::fmt;

#[derive(Debug, Clone)]
pub enum NodeType {
    SELECT,
    FROM,
    FIELD,
    TABLE,
    ROOT,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeType,
    pub value: String,
    pub childrens: Vec<Node>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(kind: NodeType, value: String) -> Self {
        let childrens: Vec<Node> = Vec::new();
        Self { kind, value, childrens }
    }

    pub fn push(&mut self, node: Node) {
        self.childrens.push(node);
    }

    pub fn last_mut(&mut self) -> Option<&mut Node> {
        self.childrens.last_mut()
    }

    pub fn print(&self, tabs: usize) {
        for _ in 0..tabs {
            print!("-");
        }

        println!("{}({})", self.kind.to_string(), self.value);

        for child in &self.childrens {
            child.print(tabs + 1);
        }
    }

    pub fn show(&mut self) {
        self.print(0);
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::SELECT => write!(f, "SELECT"),
            NodeType::FROM => write!(f, "FROM"),
            NodeType::FIELD => write!(f, "FIELD"),
            NodeType::TABLE => write!(f, "TABLE"),
            NodeType::ROOT => write!(f, "ROOT"),
        }
    }
}
