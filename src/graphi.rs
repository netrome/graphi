#[derive(Clone, Debug)]
struct Transmitter(f64, usize, usize);

#[derive(Debug)]
struct Updater(f64, usize, usize);

#[derive(Debug)]
struct Multiplier(f64, usize, usize);

#[derive(Clone, Debug)]
struct Node(f64);

#[derive(Debug)]
pub struct GraphiTMU{
    transmitters: Vec<Transmitter>,
    multipliers: Vec<Multiplier>,
    updaters: Vec<Updater>,
    nodes: Vec<Node>,
}

impl<'a> GraphiTMU{
    pub fn new() -> Self{
        Self{ transmitters: Vec::new(), multipliers: Vec::new(), updaters: Vec::new(), nodes: Vec::new() }
    }

    pub fn update(&mut self){
        let mut nodes2: Vec<Node> = self.nodes.iter().map(|_| Node(0.)).collect();
        let mut trans2 = self.transmitters.clone();

        self.transmitters.iter().for_each(|i| nodes2[i.2].0 += self.nodes[i.1].0 * i.0);
        self.multipliers.iter().for_each(|i| nodes2[i.2].0 *= self.nodes[i.1].0 * i.0);
        self.updaters.iter().for_each(|i| trans2[i.2].0 += self.nodes[i.1].0 * i.0);

        self.nodes = nodes2;
        self.transmitters = trans2;
    }
    
    pub fn push_node(&mut self, a: f64){
        self.nodes.push(Node(a));
    }

    pub fn push_transmitter(&mut self, a: f64, b: usize, c: usize){
        self.transmitters.push(Transmitter(a, b, c));
    }

    pub fn push_multiplier(&mut self, a: f64, b: usize, c: usize){
        self.multipliers.push(Multiplier(a, b, c));
    }

    pub fn push_updater(&mut self, a: f64, b: usize, c: usize){
        self.updaters.push(Updater(a, b, c));
    }

    pub fn node(&'a mut self, idx: usize) -> &'a mut f64{
        &mut self.nodes[idx].0
    }
}

