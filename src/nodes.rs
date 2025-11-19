// points and other info to be stored in Nodes structure, hopefully will make bounding boxes and hierarchy stuff easier down the line
pub struct Nodes<const D: usize> {
    pub points: Vec<[f64; D]>,
    // pub weights: Vec<[f64; 1]>,
    // pub normals: Vec<[f64; D]>, -- all this information is going to be filled in from the julia code
}

impl<const D: usize> Nodes<D> {
    
    // thinking the info gets sent in and then a Nodes structure is built from it here
    pub fn new(points: Vec<[f64; D]>) -> Self {
        Self { points } // add method for weights and normals when we get there
    }

    // right now just need to be able to pull out point info 
    pub fn point(&self, i: usize) -> &[f64; D] {
        &self.points[i]
    }
}