use rand::distributions::{Distribution, Uniform};

use super::node::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Genome{
    nodes: Vec<Node>,
    len: usize,
}

impl Genome{
    pub fn new_random(len: usize, min: u64, max: u64) -> Self{
        let mut nodes: Vec<Node> = vec![];
        for _i in 0..len {
            let new_node = Node::new_random(min, max);
            if !nodes.contains(&new_node) {
                nodes.push(new_node);
            }
        }
        Genome{nodes, len}
    }

    pub fn new(nodes: Vec<Node>) -> Self{
        let len = nodes.len();
        Genome{nodes, len}
    }

    pub fn fitness_f(&self) -> f64{
        let mut result = 0.0;
        let len = self.nodes.len();

        for i in 0..len-1 {
            result += distance(self.nodes[i],self.nodes[i+1]);
        }

        result += distance(self.nodes[0], self.nodes[len-1]);
        result
    }

    fn pick_mutations(&self, n: u32) -> Vec<[usize;2]> {
        let mut result: Vec<[usize;2]> = vec![];
        let between = Uniform::new(0, self.len);
        let mut rng = rand::thread_rng();

        let mut i = 0;
        while i < n {
            let x = between.sample(&mut rng);
            let y = between.sample(&mut rng);

            let point = [x,y];
            
            if !result.contains(&point){
                result.push(point);
                i+=1;
            }
        }
        result
    }

    pub fn mutate_random(&mut self, n: u32) -> () {
        let mutations = Genome::pick_mutations(self, n);
        self.mutate(mutations);
    }

    fn mutate(&mut self, mutations:Vec<[usize;2]>) -> () {
        mutations.iter().for_each(|&x| self.nodes.swap(x[0],x[1]));
    }

    pub fn get_len(&self) -> usize{
        self.len
    }

    pub fn get_nodes(&self) -> Vec<Node>{
        self.nodes.clone()
    }

    pub fn as_string(&self) -> String{
        let space = " | ";
        let mut result = String::from("Genome: ");

        self.nodes.iter().for_each(|&x| {
            result.push_str(&x.as_string());
            result.push_str(&space)
        });

        result
    }

    pub fn as_string_with_fitnes(&self) -> String{
        let mut result = self.as_string();
        result.push_str(&"=> ");
        result.push_str(&self.fitness_f().to_string());
        result
    }
}

pub fn crossover_2ofsprings(genome1: &Genome, genome2: &Genome, start: usize, end: usize) -> (Genome, Genome){
    if  !(genome1.len != genome2.len || 
        start >=end || 
        genome1.len < end ||  
        genome1.len < start)
    {
        let len = genome1.len;
        let mut offspring_nodes_1 = genome1.nodes[start..=end].to_vec();
        let mut offspring_nodes_2 = genome2.nodes[start..=end].to_vec();

        crossover(end, start, len, &mut offspring_nodes_1, genome2);
        crossover(end, start, len, &mut offspring_nodes_2, genome1);
        (Genome::new(offspring_nodes_1), Genome::new(offspring_nodes_2))
    }
    else{
        panic!("provide corect genoms, and mutation range");
    }
}

fn crossover(end: usize, start: usize, len: usize, offspring_nodes_1: &mut Vec<Node>, genome2: &Genome) {
    let mut position = (end-start) + 1;
    for i in end+1..len{
        if !offspring_nodes_1.contains(&genome2.nodes[i]) {
            offspring_nodes_1.insert(position,genome2.nodes[i]);
            position = (position + 1) % end;
        }
    }

    for i in 0..end+1{
        if !offspring_nodes_1.contains(&genome2.nodes[i]) {
            offspring_nodes_1.insert(position,genome2.nodes[i]);
            position = (position + 1) % end;
        }
    }
}

pub fn same_len(population: &Vec<Genome>) -> bool{
    let first_len = match population.get(0){
        Some(genome) => genome.get_len(),
        None => panic!("couldnt get element of provided population: &Vec<Genome>"),
    };

    for genome in population.iter().skip(1){
        if genome.get_len() != first_len{
            return false
        }
    }
    true
}

pub fn fitness_f(nodes: Vec<&Node>) -> f64{
    let mut result = 0.0;
    let len = nodes.len();

    for i in 0..len-1 {
        result += distance(*nodes[i],*nodes[i+1]);
    }

    result += distance(*nodes[0], *nodes[len-1]);
    result
}


//#[test]
//fn test_generation(){
//    println!("test_generation {:?}",Genome::new_random(10, 0, 10));
//}

#[test]
fn test_compare1(){
    let warsaw = Node::new(1.0,1.0);
    let paris = Node::new(2.0,2.0);

    let chromosome1 = Genome::new(vec![warsaw,paris]);
    let chromosome2 = Genome::new(vec![warsaw,paris]);

    assert_eq!(chromosome1 == chromosome2, true);
}
#[test]
fn test_compare2(){
    let warsaw = Node::new(1.0,1.0);
    let paris = Node::new(2.0,2.0);
    let povis = Node::new(2.0,3.0);


    let chromosome1 = Genome::new(vec![warsaw,paris]);
    let chromosome2 = Genome::new(vec![warsaw,povis]);

    assert_eq!(chromosome1 == chromosome2, false);
}

#[test]
fn test_fitness(){
    let warsaw = Node::new(1.0,1.0);
    let paris = Node::new(1.0,2.0);
    let povis = Node::new(1.0,3.0);

    let chromosome = Genome::new(vec![warsaw,paris,povis]);
    assert_eq!(chromosome.fitness_f() == 4.0, true);
}

#[test]
fn test_mutate(){
    let warsaw = Node::new(1.0,1.0);
    let paris = Node::new(1.0,2.0);
    let povis = Node::new(1.0,3.0);
    let wieden = Node::new(1.0,4.0);
    let vengard = Node::new(1.0,4.0);

    let mut chromosome = Genome::new(vec![warsaw,paris,povis,wieden,vengard]);
    let chromosome_before = Genome::new(vec![paris,warsaw,wieden,povis,vengard]);

    let mutation: Vec<[usize;2]> = vec![[0,1],[2,3]];
    chromosome.mutate(mutation);

    assert_eq!(chromosome_before == chromosome, true);
}

#[test]
fn test_mutate2(){
    let warsaw = Node::new(1.0,1.0);
    let paris = Node::new(1.0,2.0);
    let povis = Node::new(1.0,3.0);
    let wieden = Node::new(1.0,4.0);
    let vengard = Node::new(1.0,4.0);

    let mut chromosome = Genome::new(vec![warsaw,paris,povis,wieden,vengard]);
    let chromosome_before = Genome::new(vec![paris,povis,wieden,vengard,warsaw]);
    let mutation: Vec<[usize;2]> = vec![[0,1],[1,2],[2,3],[3,4]];
    chromosome.mutate(mutation);

    assert_eq!(chromosome_before == chromosome, true);
}


#[test]
fn test_crossover(){
    let _1 = Node::new(1.0,1.0);
    let _2 = Node::new(1.0,2.0);
    let _3 = Node::new(1.0,3.0);
    let _4 = Node::new(1.0,4.0);
    let _5 = Node::new(1.0,5.0);
    let _6 = Node::new(1.0,6.0);
    let _7 = Node::new(1.0,7.0);
    let _8 = Node::new(1.0,8.0);

    let parent_1 = Genome::new(vec![_3,_4,_8,_2,_7,_1,_6,_5]); 
    let parent_2 = Genome::new(vec![_4,_2,_5,_1,_6,_8,_3,_7]); 

    let (offspring_1, offspring_2) = crossover_2ofsprings(&parent_1, &parent_2, 3, 5);

    assert_eq!(offspring_1 == Genome::new(vec![_5,_6,_8,_2,_7,_1,_3,_4]), true);
    assert_eq!(offspring_2 == Genome::new(vec![_4,_2,_7,_1,_6,_8,_5,_3]), true);
}

#[test]
fn test_as_string(){
    let _1 = Node::new(1.0,1.0);
    let _2 = Node::new(1.0,2.0);
    let _3 = Node::new(1.0,3.0);
    let _4 = Node::new(1.0,4.0);
    let _5 = Node::new(1.0,5.0);
    let _6 = Node::new(1.0,6.0);
    let _7 = Node::new(1.0,7.0);
    let _8 = Node::new(1.0,8.0);

    let parent_1 = Genome::new(vec![_1,_2,_3,_4,_5,_6,_7,_8]); 
    assert_eq!(parent_1.as_string(), "Genome: (1 , 1) | (1 , 2) | (1 , 3) | (1 , 4) | (1 , 5) | (1 , 6) | (1 , 7) | (1 , 8) | ");
    println!("{:?}", parent_1.as_string());
}