pub struct StreamingStats {
    count: u32,
    m_k: f32,
    m_k_1: f32,
    v_k: f32,
    v_k_1: f32,
}

impl StreamingStats {
    // variance after https://math.stackexchange.com/questions/20593/calculate-variance-from-a-stream-of-sample-values
    pub fn new() -> Self {
        Self {
            count: 0,
            m_k: 0.0,
            m_k_1: 0.0,
            v_k: 0.0,
            v_k_1: 0.0,
        }
    }

    pub fn add_value(&mut self, x_k: f32) {
        self.count += 1;
        self.m_k_1 = self.m_k;
        self.v_k_1 = self.v_k;
        self.m_k = self.m_k_1 + (x_k - self.m_k_1) / self.count as f32;
        self.v_k = self.v_k_1 + (x_k - self.m_k_1) * (x_k - self.m_k);
    }

    pub fn avg(&self) -> f32 {
        self.m_k
    }

    pub fn variance(&self) -> f32 {
        self.v_k / self.count as f32
    }
}