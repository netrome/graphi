fn main(){
    println!("Hello graphi");

    let mut graph = GraphiTMU::new();

    graph.nodes.push(Node(0.)); //X
    graph.nodes.push(Node(0.)); //Y
    graph.nodes.push(Node(0.)); //l
    graph.nodes.push(Node(0.)); //err
    graph.nodes.push(Node(0.)); //u
    graph.nodes.push(Node(0.)); //delay

    graph.transmitters.push(Transmitter(0.1, 0, 1));
    graph.transmitters.push(Transmitter(1., 1, 3));
    graph.transmitters.push(Transmitter(-1., 5, 3));
    graph.transmitters.push(Transmitter(1., 3, 4));
    graph.transmitters.push(Transmitter(1., 2, 5));

    graph.multipliers.push(Multiplier(1., 0, 4));

    graph.updaters.push(Updater(-0.001, 4, 0));

    let patterns: Vec<f64> = (0..8).cycle().map(|i| i as f64).take(500).collect();
    let targets: Vec<f64> = (0..8).cycle().map(|i| (i as f64) * 2.).take(500).collect();

    // Train
    for (i, j) in patterns.iter().zip(targets.iter()){
        graph.nodes[0].0 = *i;
        graph.nodes[2].0 = *j;
        graph.update();
        println!("Err: {}", graph.nodes[3].0);
    }

    // Test
    for i in patterns.iter(){
        graph.nodes[0].0 = *i;
        graph.nodes[3].0 = 0.;
        graph.update();
        println!("Out: {}", graph.nodes[1].0);
    }
}

#[derive(Clone, Debug)]
struct Transmitter(f64, usize, usize);

#[derive(Debug)]
struct Updater(f64, usize, usize);

#[derive(Debug)]
struct Multiplier(f64, usize, usize);

#[derive(Clone, Debug)]
struct Node(f64);

#[derive(Debug)]
struct GraphiTMU{
    transmitters: Vec<Transmitter>,
    multipliers: Vec<Multiplier>,
    updaters: Vec<Updater>,
    nodes: Vec<Node>,
}

impl GraphiTMU{
    fn new() -> Self{
        Self{ transmitters: Vec::new(), multipliers: Vec::new(), updaters: Vec::new(), nodes: Vec::new() }
    }

    fn update(&mut self){
        let mut nodes2: Vec<Node> = self.nodes.iter().map(|_| Node(0.)).collect();
        let mut trans2 = self.transmitters.clone();

        self.transmitters.iter().for_each(|i| nodes2[i.2].0 += self.nodes[i.1].0 * i.0);
        self.multipliers.iter().for_each(|i| nodes2[i.2].0 *= self.nodes[i.1].0 * i.0);
        self.updaters.iter().for_each(|i| trans2[i.2].0 += self.nodes[i.1].0 * i.0);

        self.nodes = nodes2;
        self.transmitters = trans2;
    }
}

