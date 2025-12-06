use hmats_rs::*;

fn main() {
    const D: usize=3; //dimension needs to be set early on in computation as a const for openess -- see kenel definition
    let x = [0.0, 0.0];
    let y = [4.0, 0.0];
    let val = Laplace.eval(&x, &y);
    println!("Laplace Greens function = {:?}", val.re); // can chuck .re on it when using laplace 

    // trying out the nodes 
    let mut testpoints = Vec::new();
    testpoints.push([0.0, 0.0, 0.3]);
    testpoints.push([0.4, 0.2, 0.3]);
    testpoints.push([0.5, 0.5, 0.3]);
    testpoints.push([0.0, 0.5, 0.3]);

    let nodetest = Nodes::new(testpoints);
    println!("ith node value = {:?}", nodetest.point(2));


    fn constructor(nodes: &Nodes<D>, greensfunction: impl Kernel<D>) { // accepts anything with Kernel trait
        for i in 0..4 as usize {
            for j in 0..4 as usize {
                let coord1 = nodes.point(i);
                let coord2 = nodes.point(j);
                let laptest = greensfunction.eval(&coord1, &coord2);
                println!("{}th row, {}th column, cell value = {:?}", i, j, laptest);
            }
        }
    }

    constructor(&nodetest, Helmholtz{wavenumber: 3.0}); 
    let idx = [0,1,3];
    let bboxtest = nodetest.bbox_from_indices(&idx);

    println!("min values of the bounding box = {:?}", bboxtest.min);
    println!("centre of the bounding box = {:?}", bboxtest.centre());

}
