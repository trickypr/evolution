use crate::{
    genome::{Gene, NodeGene},
    settings::max_nodes,
};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ConnectionGene {
    inovation_number: Option<i64>,
    from: NodeGene,
    to: NodeGene,
    weight: Option<i64>,
    enabled: bool,
}

impl Gene for ConnectionGene {
    fn get_inovation_number(&self) -> i64 {
        self.inovation_number.unwrap()
    }
    fn set_inovation_number(&mut self, value: i64) {
        self.inovation_number = Some(value);
    }
}

impl ConnectionGene {
    pub fn new(from: NodeGene, to: NodeGene) -> ConnectionGene {
        ConnectionGene {
            inovation_number: None,
            from,
            to,
            weight: None,
            enabled: true,
        }
    }

    pub fn get_from(&self) -> &NodeGene {
        &self.from
    }

    pub fn set_from(&mut self, from: NodeGene) {
        self.from = from
    }

    pub fn get_to(&self) -> &NodeGene {
        &self.to
    }

    pub fn set_to(&mut self, to: NodeGene) {
        self.to = to
    }

    pub fn get_weight(&self) -> Option<i64> {
        self.weight
    }

    pub fn set_weight(&mut self, weight: i64) {
        self.weight = Some(weight);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled
    }

    pub fn equals(&self, other: &ConnectionGene) -> bool {
        self.from.equals(&other.from) && self.to.equals(&other.to)
    }

    pub fn hash_code(&self) -> i64 {
        self.from.get_inovation_number() * max_nodes() + self.to.get_inovation_number()
    }
}
