#[derive(Debug, PartialEq)]
pub struct Dna {
    val: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    val: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if "GCTA".to_string().contains(c) == false {
                return Err(i);
            }
        }
        Ok(Dna {
            val: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna_str: String = self
            .val
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => '\0',
            })
            .collect();

        Rna::new(&rna_str).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if "CGAU".to_string().contains(c) == false {
                return Err(i);
            }
        }
        Ok(Rna {
            val: rna.to_string(),
        })
    }
}
