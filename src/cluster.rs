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
    pub root_id: usize // index of root ClusterNode (len(.nodes)-1)
}

impl<const D: usize> ClusterTree<D> {

    // recursive ClusterNode builder from indices into Nodes
    fn build_nodes(&mut self, nodes: &Nodes<D>, mut indices: Vec<usize>, level: u32, leaf_size: usize) -> usize { 

        // create a bounding box of given indices 
        let bbox: BBox<D> = nodes.bbox_from_indices(&indices); // will break if indices empty, see Nodes impl

        // check if alrady contains max number of points  -> terminate branch (== leaf node)
        if indices.len() <= leaf_size {
            let root_id: usize = self.nodes.len();
            self.nodes.push( ClusterNode{ bbox, indices, children: None, level});
            return root_id;
        }

        // find longest dim to split down
        let longest_dim: usize = (0..D) // range 
            .max_by(|&a, &b| { // compare dims borrowed from longest_dims
                let len_a: f64 = bbox.max[a] - bbox.min[a]; 
                let len_b: f64 = bbox.max[b] - bbox.min[b];
                len_a.partial_cmp(&len_b).unwrap() // return ordering 
            })
            .unwrap();

        // directly sort indices as no need for them to be unsorted later on -- saves allocation
        indices.sort_by(|&i, &j| {
            nodes.points[i][longest_dim] // sort indices along longest_dim
                .partial_cmp(&nodes.points[j][longest_dim]) // comparing i against j indices
                .unwrap() // extract Ordering for sort_by
        });

        // bisect around mid
        let mid: usize = indices.len() / 2;
        let left_indices: Vec<usize> = indices[..mid].to_vec();
        let right_indices: Vec<usize> = indices[mid..].to_vec();

        // recursion to build children bboxes, from left and right indices
        let left_id: usize = self.build_nodes(nodes, left_indices, level + 1, leaf_size);
        let right_id: usize = self.build_nodes(nodes, right_indices, level + 1, leaf_size);
        // spacetime scrunch

        // return total number of produced nodes - 1 ie. index of root ClusterNode
        let root_id: usize = self.nodes.len(); // taken before last push so can be used as index

        // add this ClusterNode to ClusterTree, children ClusterNodes also added recursively
        self.nodes.push(ClusterNode { bbox, indices: indices, children: Some([left_id, right_id]), level});

        root_id
    }

    pub fn build_tree(nodes: &Nodes<D>, leaf_size: usize) -> Self {

        let mut tree: ClusterTree<D> = ClusterTree { nodes: Vec::new(), root_id:0};

        // for full tree, simply run build_nodes with all Node indices
        let all_indices: Vec<usize> = (0..nodes.points.len()).collect();
        let root_id: usize = tree.build_nodes(nodes, all_indices, 0, leaf_size);
        tree.root_id = root_id;
        tree
    }

    // tree build printer
    pub fn print(&self) {
        self.print_node(self.root_id, 0);
        println!("Total nodes: {}", self.nodes.len());
        println!("Root ID: {}", self.root_id);
    }

    fn print_node(&self, node_id: usize, depth: usize) {
        let node: &ClusterNode<D> = &self.nodes[node_id];
        let indent: String = "  ".repeat(depth);
        println!(
            "{}Node {}: level={}, points={}, leaf={}",
            indent,
            node_id,
            node.level,
            node.indices.len(),
            node.children.is_none()
        );

        if let Some([left, right]) = node.children {
            self.print_node(left, depth + 1);
            self.print_node(right, depth + 1);
        }
    }

}

#[cfg(test)] 
mod ClusterTree_tests { 

    use super::*; 

    // separate tests for each input error? 
    // think of things which could go wrong?
    // ultimately the aim is to have checks to see if changes to the function break things 
    // come up with list 

    #[test]
    fn leaf_size_oob() { 
        // check that leaf being too big or too small breaks the function correctly
    }

    // bbox from indices function checks for things being too small
    // should this be a test here ? 
}
