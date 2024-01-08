use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};

pub struct SemanticSimilarity {
    model: SentenceEmbeddingsModel,
}

impl SemanticSimilarity {
    pub fn default() -> Self {
        Self {
            model: SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
                .create_model()
                .unwrap(),
        }
    }
}

impl SemanticSimilarity {
    pub fn compare(&self, text_a: &str, text_b: &str) -> f64 {
        let vectors = self.model.encode(&[text_a, text_b]).unwrap();
        cosine_distance(&vectors[0], &vectors[1])
    }

    pub fn encode(&self, text_list: &Vec<String>) -> Vec<Vec<f32>> {
        self.model.encode(text_list).unwrap()
    }

    pub fn cosine_distance(&self, vec1: &Vec<f32>, vec2: &Vec<f32>) -> f64 {
        cosine_distance(vec1, vec2)
    }
}

fn cosine_distance(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f64 {
    let dot_product = dot_product(vec1, vec2);
    let root_sum_square1 = root_sum_square(vec1);
    let root_sum_square2 = root_sum_square(vec2);
    return dot_product as f64 / (root_sum_square1 * root_sum_square2);
}

fn root_sum_square(vec: &Vec<f32>) -> f64 {
    let mut sum_square = 0f32;
    for i in 0..vec.len() {
        sum_square += vec[i] * vec[i];
    }
    (sum_square as f64).sqrt()
}

fn dot_product(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
    let delta = vec1.len() as f32 - vec2.len() as f32;
    let shortest_vec = match delta {
        d if d < 0f32 => vec1,
        d if d > 0f32 => vec2,
        _ => vec1,
    };
    let mut dot_product = 0f32;
    for i in 0..shortest_vec.len() {
        dot_product += vec1[i] * vec2[i];
    }
    dot_product
}
