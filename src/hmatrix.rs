use num_complex::Complex64;
use crate::kernel::Kernel;
use crate::node::Nodes;
use crate::cluster::{ClusterTree, ClusterNode};
use crate::block::{BlockTree, BlockType};


// turning BlockTree into that sweet sweet Hmatrix 
// also useful methods etc  


// aptly named storage method for blocks
pub enum BlockStorage {
    Dense(DenseBlock),
    LowRank(LowRankBlock),
} // dense and lowrank for sorting during Hmatrix construction

// high resolution block
pub struct DenseBlock {
    pub rows: Vec<usize>, // indices into target Nodes 
    pub cols: Vec<usize>,  // indices into source Nodes
    pub data: Vec<Complex64>, // going to store data in row major order ie. A00 -> A0n, A10 -> A1n, etc 
    // Aij = data[i * len[cols] + j]
}

// approximation using A = UV^T, where U and V are essentially basis vectors since you can approximate the full Aij as basically linearly dependent 
// ACA algorithm needed in eval
pub struct LowRankBlock {
    pub rows: Vec<usize>, // indices into target Nodes 
    pub cols: Vec<usize>, // indices into source Nodes 

    // keep rank < min(rows, columns) == number of columns in U and V, how much resolution do you want to keep?
    pub rank: usize,

    pub u: Vec<Complex64>, // len(rows) x rank matrix
    pub v: Vec<Complex64>, // len(col) x rank matrix 
    // row major as above 
}


// wamt hmatrix to look something like this 
pub struct HMatrix<const D: usize, K: Kernel<D>> {

    // blocktree backbone
    pub block_tree: BlockTree,

    // actual Blocks 
    pub blocks: Vec<BlockStorage>,

    // kernel we want to use 
    pub kernel: K,

    // distance for near/far taken into account in Blocktree
    // nodes and stuff will be important but don't need to be stored here

    // dimensions of the matrix?
    pub n_rows: usize,
    pub n_cols: usize,
}

// Dense and ACA block construction functions -- in hmatrix impl
// blocks function to call both of above to create hmatrix -- in hmatrix impl
// both have to be in hmatrix impl to have access to Kernel and callable as a .construct

// need to implement Ax = y matvec product, for usability but also Krylov and stuff like that 
// min singular value with krylov or otherwise, determinant if possible, other cool and awesome and possibly tiring things that matrices do


// then maybe some practical storage info like vs a full resolution matrix
// some visualisations for the readme might be nice 
// maybe I'll even write C bindings 

// nothing will top the thrill of recursion working

// basic idea of matrix assembly, need to think about overall pipeline though, probably will have to whittle down the inputs
impl<const D: usize, K: Kernel<D>> HMatrix<D, K> {

    pub fn assemble(target_nodes: &Nodes<D>, source_nodes: &Nodes<D>,
        target_tree: &ClusterTree<D>, source_tree: &ClusterTree<D>,
        block_tree: BlockTree, kernel: K) -> Self {

            // rows and columns from nodes
            let n_rows: usize = target_nodes.points.len();
            let n_cols: usize = source_nodes.points.len();

            // set up blocks
            let mut blocks: Vec<BlockStorage> = Vec::new();

            // filter through the block tree for leaves, pontificating on this found in block.rs
            // thanks compsudoku
            for block in block_tree.nodes.iter().filter(|node| node.children.is_none()) {

                // extract global node indices from block associated clusternodes
                let target_node: &ClusterNode<D> = &target_tree.nodes[block.target_index];
                let source_node: &ClusterNode<D> = &source_tree.nodes[block.source_index];
                let rows: Vec<usize> = target_node.indices.clone();
                let cols: Vec<usize> = source_node.indices.clone();

                // sort into resolution function based off of block type 
                let stored_block: BlockStorage = match block.block_type {
                    BlockType::Near => {
                        // this is purely hypothetical mind you
                        let dense: DenseBlock = Self::build_dense_block(target_nodes, source_nodes, rows, cols, &kernel);
                        BlockStorage::Dense(dense)
                    }
                    BlockType::Far => {
                        let lowrank: LowRankBlock = Self::build_LR_block(target_nodes, source_nodes, rows, cols, &kernel);
                        BlockStorage::LowRank(lowrank)
                    }
                };
                blocks.push(stored_block);
            }

            Self { block_tree, blocks, kernel, n_rows, n_cols}
    }

    pub fn build_dense_block(target_nodes: &Nodes<D>, source_nodes: &Nodes<D>,
        rows: Vec<usize>, cols: Vec<usize>, kernel: &K) -> DenseBlock {

        // preallocation is a lovely thing
        let m: usize = rows.len();
        let n: usize = cols.len();
        let mut data: Vec<Complex64> = Vec::with_capacity(m * n);

        // a war with the borrow checker was fought here
        for &i in &rows { // row major 
            let xi: &[f64; D] = &target_nodes.points[i];
            for &j in &cols {
                let yj: &[f64; D] = &source_nodes.points[j];
                data.push(kernel.eval(xi, yj));
            }
        }
        DenseBlock {rows, cols, data}
    }


    pub fn build_LR_block(target_nodes: &Nodes<D>, source_nodes: &Nodes<D>,
        rows: &Vec<usize>, cols: &Vec<usize>, kernel: &K) -> LowRankBlock {

    }


}

