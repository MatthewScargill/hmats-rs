use crate::node::BBox;
use crate::cluster::{ClusterNode, ClusterTree};

// convert ClusterTree to BlockTree with admissibility checks 

// categorise as near for full resolution and far for approximation
pub enum BlockType {
    Near, // dense
    Far, // approximation
}

// don't think i really need D for this because by the time it gets here we've funneled it down a fair bit
// can the logic be D free / rely only on impl logic further up the line
 
 // source and target will later translate to column and row 
pub struct BlockNode{
    pub target_index: usize, // index of Cluster node in source Ctree
    pub source_index: usize, // index of Cluster node in target Ctree
    pub children: Option<Vec<usize>>, // normally another node but use Option as could be None
    // the above is going to take some sorting but this is the idea
    pub block_type: BlockType // assigned to leaf blocks 
}

pub struct BlockTree {
    pub nodes: Vec<BlockNode>,
    pub id: usize
}

pub fn is_far<const D: usize>(source_bbox: &BBox<D>, target_bbox: &BBox<D>, max_dist: f64) -> bool {
    let dist: f64 = BBox::bbox_distance(source_bbox, target_bbox);
    dist < max_dist // True if close enough for full resolution
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
            let is_far: bool = is_far(&source_bbox, &target_bbox, max_dist);

            // check if blocks are leaves or not (as will only make block if both are leaves)
            let target_is_leaf: bool = target_cluster.children.is_none();
            let source_is_leaf: bool = source_cluster.children.is_none();

            // 3 possible cases: 
            // far away from each other -- set Far and use low rank approx later
            // close and both leaves -- set Near and full resolution later
            // close but one is not a leaf -- recursion into smaller blocks like in cluster
            
            // filter for blocks far away from each other 
            if is_far {
                let id = self.nodes.len();
                self.nodes.push(BlockNode {target_index, source_index, children: None, block_type: BlockType::Far});
                return id
            }

            // class close leaves as Near
            if target_is_leaf && source_is_leaf {
                let id = self.nodes.len();
                self.nodes.push(BlockNode {target_index, source_index, children: None, block_type: BlockType::Near});
                return id
            }

            // recursive block building yay
            else {
                0
            }
        }
    
}