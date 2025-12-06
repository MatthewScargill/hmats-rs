use crate::node::{Nodes, BBox};
// doesn't need kernel stuff as cluster just hierarchy of nodes -> implement matrix construction using Tree somewhere else 


// may need a refactor of the Nodes struct because too many "nodes" about

// individual node of the tree containing indices of a fraction of the total nodes
// relationships between nodes to be kept with each node
pub struct ClusterNode<const D: usize> {
    pub bbox: BBox<D>, // bounding box of this node
    pub indices: Vec<usize>,  // indices into Nodes.points
    pub children: Option<[usize; 2]>, // indices into ClusterTree.nodes
    pub level: u32, // how many splits have we had to this box
    //pub center: [f64; D] // centre of the BBox for distance checking between boxes
}

// no build cluster node in its own impl as can't do recursion that way 

// just a collection of cluster nodes, read of relationships from individual nodes
pub struct ClusterTree<const D: usize> {
    pub nodes: Vec<ClusterNode<D>>,
}

impl<const D: usize> ClusterTree<D> {
    // tree builder which will recursively split until each "tree tip" reaches a certain amount of points 
    // lump all the cluster nodes together with logical indices 

    fn build_node(&mut self, nodes: &Nodes<D>, indices: Vec<usize>, level: u32, leaf_size: usize) -> usize { 

        // create a bounding box of given indices
        let bbox = nodes.bbox_from_indices(&indices);

        // check if alrady contains max number of points ("leaf size" says the lit) -> terminate branch 
        if indices.len() <= leaf_size {
            let id = self.nodes.len();
            self.nodes.push( ClusterNode{ bbox, indices, children: None, level});
            return id;
        }

        // finding dim and length of side to split down
        let mut longest_dim = 0;
        let mut longest_len = 0.0;
        for d in 0..D {
            let len = bbox.max[d] - bbox.min[d];
            if len > longest_len {
                longest_len = len;
                longest_dim = d;
            }
        }
        // leaves us with longest dimension of the bbox, this is the dim across which we want to split

        // sort indices on either side of split to the left and right of new "sorted" array
        let mut sorted = indices;
        sorted.sort_by(|&i, &j| { // i and j are the indices being compared
            nodes.points[i][longest_dim] // going through points along longest_dim 
                .partial_cmp(&nodes.points[j][longest_dim]) // comparing against other indices 
                .unwrap() // have been told always to add when working with Option
        });
        // leaves us with sorted vector with indices organised by value in longest_dim

        // splitting sorted into left and right halves for children bboxes
        let mid = sorted.len() / 2;
        let left_indices = sorted[..mid].to_vec();
        let right_indices = sorted[mid..].to_vec();

        // recursion to build children bboxes
        let left_id = self.build_node(nodes, left_indices, level + 1, leaf_size);
        let right_id = self.build_node(nodes, right_indices, level + 1, leaf_size);
        // spacetime scrunches up here

        // add this node to the tree, the same thing is happening with each of the children so by end of function we have full tree
        self.nodes.push(ClusterNode { bbox, indices: sorted, children: Some([left_id, right_id]), level});
        
        // its mostly a self mut function but feel like i should return something so here have the total number of nodes
        let id = self.nodes.len();
        id // includes parents and both children so 3 for the first half split 
    }
}