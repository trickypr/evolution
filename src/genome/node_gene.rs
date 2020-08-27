use crate::genome::Gene;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct NodeGene {
    x: Option<i32>,
    y: Option<i32>,
    inovation_number: i64,
}

impl Gene for NodeGene {
    fn get_inovation_number(&self) -> i64 {
        self.inovation_number
    }
    fn set_inovation_number(&mut self, value: i64) {
        self.inovation_number = value;
    }
}

impl NodeGene {
    pub fn new(inovation_number: i64) -> NodeGene {
        NodeGene {
            inovation_number,
            x: None,
            y: None,
        }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = Some(x);
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = Some(y);
    }

    pub fn get_y(&self) -> Option<i32> {
        self.y
    }

    pub fn get_x(&self) -> Option<i32> {
        self.x
    }

    pub fn equals(&self, other: &dyn Gene) -> bool {
        self.get_inovation_number() == other.get_inovation_number()
    }

    pub fn hash_code(&self) -> i64 {
        self.inovation_number
    }
}
