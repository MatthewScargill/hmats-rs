use crate::node::{Nodes, BBox};

// individual node of the tree containing indices of a fraction of the total nodes
// relationships between nodes to be kept with each node
pub struct ClusterNode<const D: usize> {
    pub bbox: BBox<D>,
    pub indices: Vec<usize>,  // indices into Nodes.points
    pub children: Option<[usize; 2]>, // indices into ClusterTree.nodes or None
    pub level: u32, // how many splits have we had to this box? 0 = root bbox
}

pub struct ClusterTree<const D: usize> {
    pub nodes: Vec<ClusterNode<D>>,
    pub id: usize // number of ClusterNodes in tree 
}

impl<const D: usize> ClusterTree<D> {

    // recursive ClusterNode builder from indices into Nodes
    fn build_nodes(&mut self, nodes: &Nodes<D>, indices: Vec<usize>, level: u32, leaf_size: usize) -> usize { 

        // create a bounding box of given indices 
        let bbox: BBox<D> = nodes.bbox_from_indices(&indices);

        // check if alrady contains max number of points  -> terminate branch (== leaf node)
        if indices.len() <= leaf_size {
            let id: usize = self.nodes.len();
            self.nodes.push( ClusterNode{ bbox, indices, children: None, level});
            return id;
        }

        // finding dim and length of side to split down
        let mut longest_dim: usize = 0;
        let mut longest_len: f64 = 0.0;
        for d in 0..D {
            let len: f64 = bbox.max[d] - bbox.min[d];
            if len > longest_len {
                longest_len = len;
                longest_dim = d; 
            }
        }

        let mut sorted: Vec<usize> = indices;
        sorted.sort_by(|&i, &j| {
            nodes.points[i][longest_dim] // sort indices along longest_dim
                .partial_cmp(&nodes.points[j][longest_dim]) // comparing i against j indices
                .unwrap() // extract Ordering for sort_by
        });

        // bisect sorted
        let mid: usize = sorted.len() / 2;
        let left_indices: Vec<usize> = sorted[..mid].to_vec();
        let right_indices: Vec<usize> = sorted[mid..].to_vec();

        // recursion to build children bboxes, from left and right indices
        let left_id: usize = self.build_nodes(nodes, left_indices, level + 1, leaf_size);
        let right_id: usize = self.build_nodes(nodes, right_indices, level + 1, leaf_size);
        // spacetime scrunches up here

        // return total number of produced nodes - 1
        let id: usize = self.nodes.len(); // taken before last push so can be used as index

        // add this ClusterNode to ClusterTree, children ClusterNodes also added recursively
        self.nodes.push(ClusterNode { bbox, indices: sorted, children: Some([left_id, right_id]), level});
        
        id 
    }

    pub fn build_tree(nodes: &Nodes<D>, leaf_size: usize) -> Self {

        let mut tree: ClusterTree<D> = ClusterTree { nodes: Vec::new(), id:0};

        // for full tree, simply run build_nodes with all Node indices
        let all_indices: Vec<usize> = (0..nodes.points.len()).collect();
        let id: usize = tree.build_nodes(nodes, all_indices, 0, leaf_size);
        tree.id = id;
        tree
    }

    // dumb tree build tester
    pub fn print(&self) {
        let mut i = 0;
        for node in &self.nodes {
            println!("level: {}, index: {}",&node.level, i); i += 1;
        }
        println!("self id: {}", &self.id)
    }
}
