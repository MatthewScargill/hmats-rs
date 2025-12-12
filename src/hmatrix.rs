use crate::kernel::Kernel;
use crate::cluster::ClusterTree;
use crate::block::BlockTree;


// turning BlockTree into that sweet sweet Hmatrix 
// also useful methods etc  


// wamt hmatrix to look something like this 
pub struct HMatrix<const D: usize, K: Kernel<D>> {

    // blocktree backbone
    pub block_tree: BlockTree,
    // kernel we want to use 
    pub kernel: K,

    // distance for near/far taken into account in Blocktree
    // nodes and stuff will be important but don't need to be stored here

    // dimensions of the matrix?
    pub n_rows: usize,
    pub n_cols: usize,

    // ofc need to figure out how to store actual blocks 
    
}