use crate::node::BBox;
//use crate::cluster::{ClusterNode,ClusterTree};

// convert ClusterTree to BlockTree with admissibility checks 

// categorise as near for full resolution and far for approximation
pub enum BlockType {
    Near, // dense
    Far,  // approximation
}

// don't think i really need D for this because by the time it gets here we've funneled it down a fair bit
// can the logic be D free / rely only on impl logic further up the line
 
pub struct BlockNode{
    pub t: usize, // index of Cluster node in source Ctree
    pub s: usize, // index of Cluster node in target Ctree
    pub children: Option<Vec<usize>>, // normally another node but use Option as could be None
    // the above is going to take some sorting but this is the idea
    pub block_type: BlockType // assigned to leaf blocks 
}

// same idea as the cluster tree just converted to block nodes 
pub struct BlockTree {
    pub nodes: Vec<BlockNode>,
    pub id: usize
}

pub fn admissible<const D: usize>(source_bbox: &BBox<D>, target_bbox: &BBox<D>, max_dist: f64) -> bool {
    let dist: f64 = BBox::bbox_distance(source_bbox, target_bbox);
    dist < max_dist // True if close enough for full resolution
}