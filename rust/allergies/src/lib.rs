use strum::IntoEnumIterator;
use strum_macros::EnumIter; 

pub struct Allergies
{
    score: u32
}

#[derive(Copy, Clone, EnumIter, Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies{
            score : score % 256
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score>0 && (self.score & allergen.clone() as u32) == 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut algs = vec![];
        let mut score = self.score;
        let base:u8 = 2;
        let algo_score = |alg:Allergen| base.pow(alg as u32) as u8;

        for alg in Allergen::iter().rev()
        {
            let cur_score:u8 = algo_score(alg);
            if score >= cur_score as u32 {
                algs.push(alg);
                score = score - cur_score as u32;
            }
        }
        algs
    }
}
