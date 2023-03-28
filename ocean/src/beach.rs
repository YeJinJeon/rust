use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    pub crab_vec: Vec<Crab>
}

impl Beach {
    pub fn new() -> Beach {
        Beach{
            crab_vec: Vec::new()
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crab_vec.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crab_vec.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crab_vec[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crab_vec.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        let mut speed = 0;
        let mut fastest_crab = None;
        for c in self.crabs() {
            if c.speed >= speed{
                fastest_crab = Some(c);
                speed = c.speed;
            }
        }
        fastest_crab
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut crab_vec_by_name = Vec::new();
        for c in self.crabs() {
            if c.name == name{
               crab_vec_by_name.push(c)
            }
        }
        crab_vec_by_name
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let p1 = &self.crab_vec[i];
        let p2 = &self.crab_vec[j];
        self.crab_vec.push(Crab::breed(p1, p2, name))
    }
}
