// points and other info to be stored in Nodes structure, hopefully will make bounding boxes and hierarchy stuff easier down the line
pub struct Nodes<const D: usize> {
    pub points: Vec<[f64; D]>,
    // pub weights: Vec<[f64; 1]>,
    // pub normals: Vec<[f64; D]>, -- all this information is going to be filled in from the julia code
}

impl<const D: usize> Nodes<D> {
    
    // thinking the info gets sent in and then a Nodes structure is built from it here
    pub fn new(points: Vec<[f64; D]>) -> Self {
        Self { points} // add method for weights and normals when we get there
    }

    // right now just need to be able to pull out point info 
    pub fn point(&self, i: usize) -> &[f64; D] {
        &self.points[i]
    }
}

// bounding boxes are the backbone of the method, each one gets subdivided into more bounding boxes to form clusters, belonging to a tree (spoilers)
#[derive(Debug, Clone, Copy)] // will need lots
pub struct BBox<const D: usize> {
    pub min: [f64; D], // (x_min, y_min, ...)
    pub max: [f64; D], // (x_max, y_max, ...)
}

impl <const D: usize> BBox<D> {

    // obviously need to able to define one from a set of points 
    pub fn from_points(pts: &[[f64; D]]) -> Self {
        assert!(!pts.is_empty()); // need points

        // find min in each dimension of all points to build global min and max for bounding box
        let mut min: [f64; D] = pts[0]; 
        let mut max: [f64; D] = pts[0];
        for p in pts.iter().skip(1) {
            for d in 0..D {
                if p[d] < min[d] {min[d] = p[d]} // replace with new min
                if p[d] > max[d] {max[d] = p[d]} // you're way ahead of me
            }
        }
        Self { min, max} 
    }

    pub fn centre(&self) -> Vec<f64>{ 

        let mut centre: Vec<f64> = Vec::new();
        let dim = D as f64;

        for d in 0..D {
            let centre_i = (self.min[d] + self.max[d])/ dim ;
            centre.push(centre_i);
        }
        centre 
    }

}

// rehash some stuff as paper says from indices is the way to go 
impl <const D: usize> Nodes<D> {
    pub fn bbox_from_indices(&self, idxs: &[usize]) -> BBox<D> {
        assert!(!idxs.is_empty());
        let mut min: [f64; D] = self.points[idxs[0]];
        let mut max: [f64; D] = self.points[idxs[0]];
        for &i in idxs.iter().skip(1) {
            let p = self.points[i];
            for d in 0..D {
                if p[d] < min[d] { min[d] = p[d]}
                if p[d] > max[d] { max[d] = p[d]}
            }
        }
        BBox { min, max}
    }
}