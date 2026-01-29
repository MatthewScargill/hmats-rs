use crate::nodes::BBox;
use crate::cluster::{ClusterNode, ClusterTree};


// comments:
// - sometimes i wonder if we need the full block tree and node just a collection of the block leaves 
// the only thing i can think of is changing the max rank on the fly? maybe for k scanning that would be good?
// then again matrix methods might require the full tree, either way i like it, let's keep it moving

// categorise as near for full resolution and far for approximation
pub enum BlockType {
    Near, // dense
    Far, // approximation
}

// don't think i really need D for this because by the time it gets here we've funneled it down a fair bit
// can the logic be D free / rely only on impl logic further up the line

 // source and target will later translate to column and row 
pub struct BlockNode{
    pub target_index: usize, // index of Cluster node in target Ctree
    pub source_index: usize, // index of Cluster node in source Ctree
    pub children: Option<Vec<usize>>, // normally another node but use Option as could be None
    // the above is going to take some sorting but this is the idea
    pub block_type: BlockType // assigned to leaf blocks 
}

pub struct BlockTree {
    pub nodes: Vec<BlockNode>,
    pub root_id: usize
}

// this should be switched our for something that takes bbox size into account 
pub fn is_far<const D: usize>(source_bbox: &BBox<D>, target_bbox: &BBox<D>, max_dist: f64) -> bool {
    let dist: f64 = BBox::bbox_distance(source_bbox, target_bbox);
    dist > max_dist // True if too far for full resolution
}

impl BlockTree {
    
    fn build_blocks<const D: usize>( &mut self, target_index: usize, source_index: usize, 
        target_tree: &ClusterTree<D>, source_tree: &ClusterTree<D>, max_dist: f64) -> usize {

            // find corresponding ClusterNode and associated bboxes
            let target_cluster: &ClusterNode<D> = &target_tree.nodes[target_index];
            let source_cluster: &ClusterNode<D> = &source_tree.nodes[source_index];

            let target_bbox: &BBox<D> = &target_cluster.bbox;
            let source_bbox: &BBox<D> = &source_cluster.bbox;

            // proximity check
            let too_far: bool = is_far(source_bbox, target_bbox, max_dist);

            // check if blocks are leaves 
            let target_is_leaf: bool = target_cluster.children.is_none();
            let source_is_leaf: bool = source_cluster.children.is_none();

            // 3 possible cases: 
            // far away from each other -- set Far and use low rank approx later
            // close and both leaves -- set Near and full resolution later
            // close but one is not a leaf -- recursion into smaller blocks like in cluster
            
            // filter for blocks far away from each other 
            if too_far {
                let root_id: usize = self.nodes.len();
                self.nodes.push(BlockNode {target_index, source_index, children: None, block_type: BlockType::Far});
                return root_id
            }

            // class close leaves as Near
            if target_is_leaf && source_is_leaf {
                let root_id: usize = self.nodes.len();
                self.nodes.push(BlockNode {target_index, source_index, children: None, block_type: BlockType::Near});
                return root_id
            }

            // recursive block building yay
            else { // ie close and not leaves

                // child indices 
                let mut child_indices: Vec<usize> = Vec::new();

                match (target_cluster.children ,source_cluster.children) {

                    // both clusters have children
                    (Some([tc0, tc1]), Some([sc0, sc1])) => {
                        for &tci in &[tc0, tc1] {
                            for &sci in &[sc0, sc1] {
                                let child: usize = self.build_blocks(tci, sci, target_tree, source_tree, max_dist);
                                child_indices.push(child);
                            }
                        }
                    }

                    // the source cluster is a leaf
                    (Some([tc0, tc1]), None) => {
                        for &tci in &[tc0, tc1] {
                            let child = self.build_blocks(tci, source_index, target_tree, source_tree, max_dist);
                            child_indices.push(child);
                        }
                    }

                    // the target cluster is a leaf
                    (None, Some([s0, s1])) => {
                        for &sci in &[s0, s1] {
                            let child = self.build_blocks(target_index, sci, target_tree, source_tree, max_dist);
                            child_indices.push(child);
                        }
                    }

                    (None, None) => unreachable!("Leaf leaf interaction taken care of above."),
                }
                // lovely lovely spacetime crunch

                // return number of BlockNodes created from above (target_indices, source_indices) - 1
                let root_id: usize = self.nodes.len(); // taken before last push so can be used as index

                // add BlockNode to Blocktree 
                self.nodes.push(BlockNode { target_index, source_index, children: Some(child_indices), block_type: BlockType::Near });

                root_id
            }
        }

        pub fn build_tree<const D: usize>(target_tree: &ClusterTree<D>, source_tree: &ClusterTree<D>, max_dist: f64) -> Self {

            let mut tree: BlockTree = BlockTree { nodes: Vec::new(), root_id: 0 };

            let target_index: usize = target_tree.root_id;
            let source_index: usize = source_tree.root_id;

            let root_id: usize = tree.build_blocks(target_index, source_index, target_tree, source_tree, max_dist);

                
            print!("blocktree succesfully built");

            tree.root_id = root_id;
            tree
        }
}
