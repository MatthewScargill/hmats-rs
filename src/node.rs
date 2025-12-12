use distances::vectors::euclidean;

// Nodes structure to hold points (and later boundary data in the case of dG problems)
pub struct Nodes<const D: usize> {
    pub points: Vec<[f64; D]>,
    // pub weights: Vec<[f64; 1]>,
    // pub normals: Vec<[f64; D]>, 
}

impl<const D: usize> Nodes<D> {
    
    // create Nodes from "standard" (Vec<[f64; D]>) points
    pub fn new(points: Vec<[f64; D]>) -> Self {
        assert!(!points.is_empty());
        Self { points} 
        // add method for weights and normals if/when dG implemented
    }

    // creating bounding boxes from indices allows the reuse of Nodes structure instead of storing subdivisions of Nodes.points
    pub fn bbox_from_indices(&self, indices: &[usize]) -> BBox<D> {

        assert!(!indices.is_empty()); 

        let mut min: [f64; D] = self.points[indices[0]];
        let mut max: [f64; D] = self.points[indices[0]];

        for &i in indices.iter().skip(1) { 
            let p: [f64; D] = self.points[i];
            for d in 0..D { // finding min/max over each spatial dimension
                if p[d] < min[d] { min[d] = p[d]}
                if p[d] > max[d] { max[d] = p[d]}
            }
        }
        BBox { min, max}
    }
}

// Bounding boxes are used to subdivide Nodes and later provide notion of distance for admissibility in block.rs
#[derive(Debug, Clone, Copy)] 
pub struct BBox<const D: usize> {
    pub min: [f64; D], // (x_min, y_min, ...)
    pub max: [f64; D], // (x_max, y_max, ...)
}

impl <const D: usize> BBox<D> {

    pub fn centre(&self) -> Vec<f64>{ 

        let mut centre: Vec<f64> = Vec::new();
        let dim: f64 = D as f64;

        for d in 0..D {
            let centre_i: f64 = (self.min[d] + self.max[d])/ dim ;
            centre.push(centre_i);
        }
        centre 
    }

    pub fn bbox_distance(source_bbox: &BBox<D>, target_bbox: &BBox<D>) -> f64 {
        
        let source_centre: Vec<f64> = source_bbox.centre();
        let target_centre: Vec<f64> = target_bbox.centre();
        let distance: f64 = euclidean(&source_centre, &target_centre);

        distance
    }
}
