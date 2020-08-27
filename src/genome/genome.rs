use crate::{
    data_structures::{random_hash_set, RandomHashSet},
    genome::{ConnectionGene, NodeGene},
    neat::Neat,
};

pub struct Genome {
    conections: RandomHashSet<ConnectionGene>,
    nodes: RandomHashSet<NodeGene>,
    neat: Neat,
}

impl Genome {
    pub fn new(neat: Neat) -> Genome {
        Genome {
            conections: random_hash_set::new::<ConnectionGene>(),
            nodes: random_hash_set::new::<NodeGene>(),
            neat,
        }
    }

    pub fn distances(&self, g2: &Genome) {}

    pub fn crossover(g1: Genome, g2: Genome) {}

    pub fn mutate(&mut self) {}

    pub fn get_connection(&mut self) -> &mut RandomHashSet<ConnectionGene> {
        &mut self.conections
    }
    pub fn get_nodes(&mut self) -> &mut RandomHashSet<NodeGene> {
        &mut self.nodes
    }
    pub fn get_neat(&self) -> &Neat {
        &self.neat
    }

    pub fn set_connection(&mut self, conections: RandomHashSet<ConnectionGene>) {
        self.conections = conections
    }
}
