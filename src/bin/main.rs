extern crate graphi;

use graphi::graphi::GraphiTMU;

fn main(){
    println!("Hello graphi");

    let mut graph = GraphiTMU::new();

    graph.push_node(0.); //X
    graph.push_node(0.); //Y
    graph.push_node(0.); //l
    graph.push_node(0.); //err
    graph.push_node(0.); //u
    graph.push_node(0.); //delay

    graph.push_transmitter(0.1, 0, 1);
    graph.push_transmitter(1., 1, 3);
    graph.push_transmitter(-1., 5, 3);
    graph.push_transmitter(1., 3, 4);
    graph.push_transmitter(1., 2, 5);

    graph.push_multiplier(1., 0, 4);
    graph.push_updater(-0.001, 4, 0);


    let patterns: Vec<f64> = (0..8).cycle().map(|i| i as f64).take(500).collect();
    let targets: Vec<f64> = (0..8).cycle().map(|i| (i as f64) * 2.).take(500).collect();

    // Train
    for (i, j) in patterns.iter().zip(targets.iter()){
        *graph.node(0) = *i;
        *graph.node(2) = *j;
        graph.update();
        println!("Err: {}", graph.node(3));
    }

    // Test
    for i in patterns.iter(){
        *graph.node(0) = *i;
        *graph.node(3) = 0.;
        graph.update();
        println!("Out: {}", graph.node(1));
    }
}

