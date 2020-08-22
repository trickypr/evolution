use rand::Rng;

pub struct RandomSelector<T: Copy> {
    objects: Vec<T>,
    scores: Vec<f64>,
    total_score: f64,
}

pub fn new<T: Copy>() -> RandomSelector<T> {
    RandomSelector {
        objects: Vec::new(),
        scores: Vec::new(),
        total_score: 0.0,
    }
}

impl<T: Copy> RandomSelector<T> {
    pub fn add(&mut self, element: T, score: f64) {
        self.objects.push(element);
        self.scores.push(score);
        self.total_score += score;
    }

    pub fn random(&self) -> Option<T> {
        let mut rand = rand::thread_rng();

        let y = rand.gen::<f64>() * self.total_score;
        let mut c = 0.0;

        for i in 0..self.objects.len() {
            c += self.scores.get(i).unwrap();
            if c > y {
                return Some(self.objects[i]);
            }
        }

        return None;
    }

    pub fn reset(&mut self) {
        self.objects.clear();
        self.scores.clear();
        self.total_score = 0.0;
    }
}
