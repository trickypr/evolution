use std::collections::HashMap;

use crate::{
    data_structures::{random_hash_set, RandomHashSet},
    genome::{ConnectionGene, Gene, Genome, NodeGene},
    settings,
};

#[derive(Clone)]
pub struct Neat {
    all_connections: HashMap<ConnectionGene, ConnectionGene>,
    all_nodes: RandomHashSet<NodeGene>,
    input_size: i32,
    output_size: i32,
    max_clients: i32,
}

impl Neat {
    pub fn new(input_size: i32, output_size: i32, clients: i32) -> Neat {
        let mut neat = Neat {
            all_connections: HashMap::new(),
            all_nodes: random_hash_set::new(),
            input_size,
            output_size,
            max_clients: clients,
        };

        neat.reset(input_size, output_size, clients);

        neat
    }

    pub fn empty_genome(&mut self) -> Genome {
        let mut g = Genome::new(self.clone());

        for i in 0..(self.input_size + self.output_size) {
            g.get_nodes().add(&self.get_node_by_id((i + 1) as i64));
        }

        g
    }

    pub fn reset(&mut self, input_size: i32, output_size: i32, clients: i32) {
        self.input_size = input_size;
        self.output_size = output_size;
        self.max_clients = clients;

        self.all_connections.clear();
        self.all_nodes.clear();

        for i in 0..self.input_size {
            let mut n = self.get_node();
            n.set_x(10);
            n.set_y((i + 1) * 20);
        }

        for i in 0..self.output_size {
            let mut n = self.get_node();
            n.set_x(settings::node_width());
            n.set_y((i + 1) * 20);
        }
    }

    pub fn clone_connection(con: ConnectionGene) -> ConnectionGene {
        let c = ConnectionGene::new(con.get_from().clone(), con.get_to().clone());
        c.set_weight(con.get_weight());
        c.set_enabled(con.is_enabled());
        c
    }
    pub fn get_connection(&mut self, node1: NodeGene, node2: NodeGene) -> ConnectionGene {
        let connection_gene = ConnectionGene::new(node1, node2);

        let connection = self.all_connections.get(&connection_gene);
        if connection.is_some() {
            connection_gene.set_inovation_number(connection.unwrap().get_inovation_number());
        } else {
            connection_gene.set_inovation_number((self.all_connections.len() + 1) as i64);
            self.all_connections
                .insert(connection_gene, connection_gene);
        }

        connection_gene
    }

    pub fn get_node(&mut self) -> NodeGene {
        let n = NodeGene::new((self.all_nodes.size() + 1) as i64);
        self.all_nodes.add(&n);
        n
    }
    pub fn get_node_by_id(&mut self, id: i64) -> NodeGene {
        if id <= self.all_nodes.size() as i64 {
            return self.all_nodes.get((id - 1) as usize).unwrap().clone();
        }

        self.get_node()
    }
}

mod neat_core_example {
    use crate::neat::Neat;
    #[test]
    fn correct_node_count() {
        let mut neat = Neat::new(3, 3, 100);

        let mut g = neat.empty_genome();
        assert_eq!(g.get_nodes().size(), 6);
    }
}
