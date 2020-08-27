pub trait Gene {
    fn get_inovation_number(&self) -> i64;
    fn set_inovation_number(&mut self, value: i64);
}
