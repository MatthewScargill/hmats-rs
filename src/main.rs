use hmats_rs::*;
use scilib::constant::C;

fn main() {
    const D: usize=2; //dimension needs to be set early on in computation as a const for openess -- see kenel definition
    let x = [0.0, 0.0];
    let y = [4.0, 0.0];
    let val = Laplace.eval(&x, &y);
    println!("Laplace Greens function = {:?}", val.re); // can chuck .re on it when using laplace 

    // trying out the nodes 
    let mut testpoints: Vec<[f64; 3]> = Vec::new();
    testpoints.push([0.0, 0.0, 2.0]);
    testpoints.push([0.4, 0.2, 0.3]);
    testpoints.push([0.5, 0.5, 0.3]);
    testpoints.push([0.0, 1.0, 0.]);

    let nodetest = Nodes::new(testpoints);
    println!("ith node value = {:?}", nodetest.points[2]);


    let cn = cardioid_nodes(30);
    let card_nodes = Nodes::new(cn);


    
    fn constructor(nodes: &Nodes<D>, greensfunction: impl Kernel<D>) { // accepts anything with Kernel trait
        for i in 0..30 as usize {
            for j in 0..30 as usize {
                let coord1 = nodes.points[i];
                let coord2 = nodes.points[j];
                let laptest = greensfunction.eval(&coord1, &coord2);
                println!("{}th row, {}th column, cell value = {:?}", i, j, laptest);
            }
        }
    }

    constructor(&card_nodes, Helmholtz{wavenumber: 3.0});
    println!("-------------------");
    //constructor(&card_nodes, Laplace);
    let idx = [0,1,3];
    //let bboxtest: BBox<D> = nodetest.bbox_from_indices(&idx);


    //println!("min values of the bounding box = {:?}", bboxtest.min);
    //println!("centre of the bounding box = {:?}", bboxtest.centre());

    //let testclustertree: ClusterTree<D> = ClusterTree::build_tree(&nodetest, 1);

    //let _testblocktree: BlockTree = BlockTree::build_tree(&testclustertree, &testclustertree, 0.4);

    //testclustertree.print();

}
