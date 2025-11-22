//mod kernels;
//use kernels::*;

use hmats_rs::*;

fn main() {
    const D: usize= 2; //dimension needs to be set early on in computation as a const for openess -- see kenel definition
    let x = [0.0_f64, 0.0];
    let y = [4.0_f64, 0.0];
    let laplace = Laplace2D;
    let val = laplace.eval(&x, &y);
    println!("Laplace Greens function = {:?}", val);

    // trying out the nodes 
    let mut testpoints = Vec::new();
    testpoints.push([0.0, 0.0]);
    testpoints.push([0.4, 0.2]);
    testpoints.push([0.5, 0.5]);
    testpoints.push([0.0, 0.5]);

    let nodetest = Nodes::new(testpoints);
    println!("ith node value = {:?}", nodetest.point(2));


    fn constructor(nodes: &Nodes<D>, greensfunction: Laplace2D) { // need to find a way to generalise greensfunction to any type
        for i in 0..4 as usize {
            for j in 0..4 as usize {
                let coord1 = nodes.point(i);
                let coord2 = nodes.point(j);
                let laptest = Laplace2D.eval(&coord1, &coord2);
                println!("{}th row, {}th column, cell value = {:?}", i, j, laptest);
            }
        }
    }

    constructor(&nodetest, Laplace2D);

    let idx = [0,1,3];
    let bboxtest = nodetest.bbox_from_indices(&idx);

    println!("min values of the bounding box = {:?}", bboxtest.min);
    println!("centre of the bounding box = {:?}", bboxtest.centre());

}
