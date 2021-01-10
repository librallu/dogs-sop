use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::vec::Vec;

pub type CityId = u32;
pub type Cost = i32;


/// Stores data about a SOP instance
#[derive(Debug)]
pub struct Instance {
    name: String,
    n: CityId,
    matrix: Vec<Vec<Cost>>,
    predecessors: Vec<Vec<CityId>>,
    successors: Vec<Vec<CityId>>,
}

impl Instance {
    /// reads from a file a SOP instance and extracts required information to solve it.
    pub fn new(filename: &str) -> Result<Instance, Error> {
        let input = File::open(filename)?;
        let buffered = BufReader::new(input);
        let mut n = 0;
        let mut matrix: Vec<Vec<Cost>> = Vec::new();
        for (i, line) in buffered.lines().enumerate() {
            if i == 0 {
                // read first line as nb cities
                n = line.unwrap().parse::<CityId>().unwrap();
            } else {
                if (i as CityId) <= n {
                    matrix.push(
                        line.unwrap()
                            .split("\t")
                            .filter(|e| e.len() > 0)
                            .map(|e| e.parse::<Cost>().unwrap())
                            .collect(),
                    );
                }
            }
        }
        // precompute predecessors
        let mut predecessors = Vec::new();
        let mut successors = Vec::new();
        for (i, a) in matrix.iter().enumerate() {
            predecessors.push(Vec::new());
            successors.push(Vec::new());
            for (j, &b) in a.iter().enumerate() {
                if b < 0 {
                    predecessors[i].push(j as CityId);
                } else {
                    successors[i].push(j as CityId);
                }
            }
        }
        // return the instance
        Ok(Instance {
            name: filename.to_string(),
            n: n,
            matrix: matrix,
            predecessors: predecessors,
            successors: successors,
        })
    }

    pub fn nb_cities(&self) -> CityId {
        return self.n;
    }

    pub fn predecessors(&self, i: &CityId) -> &Vec<CityId> {
        return &self.predecessors[*i as usize];
    }

    pub fn possible_successors(&self, i: &CityId) -> &Vec<CityId> {
        return &self.successors[*i as usize];
    }

    pub fn cost_arc(&self, u: CityId, v: CityId) -> Cost {
        return self.matrix[u as usize][v as usize];
    }
}
