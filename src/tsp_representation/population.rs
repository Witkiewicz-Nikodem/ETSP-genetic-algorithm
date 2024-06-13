use crate::tsp_representation::population;
use float_cmp::{approx_eq, Ulps};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use itertools::Itertools;

use super::genome::{self, crossover_2ofsprings, same_len, Genome};
use super::node::*;
// population is created of points on Cartesian coordinate system but only on positive quadrant (x >= 0 and y >= 0).
// in random generation it is necessary to provide min and max value for both x and y. They are generated on rectangle x in <min,max> X y in <min,max>

#[derive(Debug)]
pub struct Population{
    population: Vec<Genome>,
    len: usize,
    genome_len: usize,
    number_of_crossovers: u32,
    number_of_mutations: u32,
    number_of_mutated_genoms: u32,
    crossover_begin_range: usize,
    crossover_end_range: usize,
}

impl Population{
    pub fn new_random(
        len:usize,
        genome_len: usize,
        min: u64,
        max: u64,
        number_of_crossovers: u32,
        number_of_mutations: u32,  
        number_of_mutated_genoms: u32,
        crossover_start: usize,
        crossover_end: usize) -> Self

        {
            let mut population: Vec<Genome> = vec![];
            let mut nodes: Vec<Vec<Node>> = vec![];
            
            let mut first = Genome::new_random(genome_len, min, max).get_nodes();
            let mut rng = rand::thread_rng();

            let mut n = 0;
            while n < len {
                first.shuffle(&mut rng);
                if !nodes.contains(&first){
                    nodes.push(first.clone());
                    n+=1;
                }
            }

            if crossover_start >= genome_len{
                panic!("crossover_start is >= genome_len");
            }

            if crossover_end > genome_len{
                panic!("crossover_end is > genome_len");
            }

            if crossover_start >= crossover_end {
                panic!("crossover_start >= crossover_end");
            }
            
            nodes.into_iter().for_each(|node| population.push(Genome::new(node)));
            Population::new(population, number_of_crossovers, number_of_mutations, number_of_mutated_genoms, crossover_start, crossover_end)
        }

    pub fn new(
        population: Vec<Genome>,
        number_of_crossovers: u32,
        number_of_mutations: u32,
        number_of_mutated_genoms: u32,
        crossover_start: usize,
        crossover_end: usize) -> Self
        {
            let (genome_len, len) = match same_len(&population){
                true => (population[0].get_len(), population.len()),
                false => panic!{"provided genoms have different lenghts"}
            };

            if (number_of_crossovers as usize) *2 > population.len(){
                panic!{"provided bigger number_of_crossovers than it is posible"};
            }

            if (number_of_mutated_genoms as usize) > population.len(){
                panic!{"provided bigger number_of_mutated_genoms than it is posible"};
            }

            Population{population, len, genome_len, number_of_crossovers, number_of_mutations, number_of_mutated_genoms, crossover_begin_range: crossover_start, crossover_end_range: crossover_end}
        }


    
    fn pick_crossovers(&self) -> Vec<(usize,usize)>{
        let mut crossovers: Vec<(usize,usize)> = vec![];        
        let mut potential_parents = self.population.clone();
        let mut potential_parents_indexes: Vec<usize> = (0..potential_parents.len()).collect();

        let between = Uniform::new(0.0, 1.0);
        let mut rng = rand::thread_rng();
        let mut roulete;
        let mut pick;
        let mut parent1;
        let mut parent2;
        let mut parent1_id;
        let mut parent2_id;

        for _ in 0..self.number_of_crossovers{
            roulete = determine_roulete(&potential_parents);
            pick = between.sample(&mut rng);
            parent1 = take_index_from_roulete(&roulete, pick);
            parent1_id = potential_parents_indexes[parent1];
            potential_parents.remove(parent1);
            potential_parents_indexes.remove(parent1);

            roulete = determine_roulete(&potential_parents);
            pick = between.sample(&mut rng);
            parent2 = take_index_from_roulete(&roulete, pick);
            parent2_id = potential_parents_indexes[parent2];
            potential_parents.remove(parent2);
            potential_parents_indexes.remove(parent2);
            
            crossovers.push((parent1_id,parent2_id));
        }
        crossovers
    }

    pub fn crossover_2ofsprings(&mut self){
        let crossovers = self.pick_crossovers();
        for (parent1,parent2) in crossovers{
            let (ofspring1,ofspring2) = crossover_2ofsprings(&self.population[parent1], &self.population[parent2], self.crossover_begin_range, self.crossover_end_range);
            if !self.population.contains(&ofspring1) {
                self.population.push(ofspring1);
            }
            if !self.population.contains(&ofspring2) {
                self.population.push(ofspring2);
            }
        }
    }

    pub fn mutate(&mut self){
        let mut potential_mutations: Vec<usize> = (1..self.population.len()).collect();
        let mut mutations: Vec<usize> = vec![];
        let mut rng = rand::thread_rng();
        let mut id;

        for _ in 0..self.number_of_mutated_genoms{
            let between = Uniform::new(0, potential_mutations.len());
            id = between.sample(&mut rng);
            mutations.push(potential_mutations[id]);
            potential_mutations.remove(id);
        }

        for i in mutations{
            self.population[i].mutate_random(self.number_of_mutations);
        }
    }

    pub fn reduce(&mut self){
        self.population.sort_by_key(|genome| genome.fitness_f().to_bits());
        self.population = self.population[0..self.len].to_vec();
    }

    pub fn as_string(&self) -> String {
        let mut result = String::from("Population: \n===========================================================================================================================\n");
        self.population.iter().for_each(|x|{
            result.push_str(&x.as_string());
            result.push('\n');
        });
        result
    }

    pub fn as_string_with_fitnes(&self) -> String {
        let mut result = String::from("Population: \n===========================================================================================================================\n");
        let arrow = "=> ";
        self.population.iter().for_each(|x|{
            result.push_str(&x.as_string());
            result.push_str(&arrow);
            result.push_str(&x.fitness_f().to_string());
            result.push('\n');
        });
        result
    }

    pub fn round(&mut self){
        self.crossover_2ofsprings();
        self.mutate();
        self.reduce();
    }

    pub fn get_best(&self) -> Genome{
        let mut result = self.population[0].clone();
        let mut result_fitness = result.fitness_f();
        self.population.iter().for_each(|x| {
            let x_fitness = x.fitness_f();
            if  x_fitness < result.fitness_f(){
                result_fitness = x_fitness;
                result = x.clone();
            }
        });
        result
    }
}


fn determine_roulete(population: &Vec<Genome>) -> Vec<f64>{ 
    let mut result: Vec<f64> = vec![];
    result.push(population[0].fitness_f());

    for i in 1..population.len(){
        result.push(population[i].fitness_f() + result[i-1]);
    }

    let sum = result[result.len()-1];
    result.iter_mut().for_each(|x| *x/= sum);
    result 
}

fn take_index_from_roulete(roulete: &Vec<f64>, pick:f64) -> usize{
    let mut i = 0;
    while i < roulete.len(){
        if pick < roulete[i]{
            return i;
        }
        i+=1;
    }
    panic!("didn't find index for pick: {:?}", pick);
}



#[test]
fn new_generation(){
    println!("generated population: {:?}", Population::new_random(15,10,0,100,3,3,3,1,2));
}

#[test]
#[should_panic(expected="provided genoms have different lenghts")]
fn new_validation_panic(){
    let _1 = Node::new(1.0,1.0);
    let _2 = Node::new(1.0,2.0);
    let _3 = Node::new(1.0,3.0);
    let _4 = Node::new(1.0,4.0);
    let _5 = Node::new(1.0,5.0);
    let _6 = Node::new(1.0,6.0);
    let _7 = Node::new(1.0,7.0);
    let _8 = Node::new(1.0,8.0);

    let parent_1 = Genome::new(vec![_3,_4,_8,_2,_7]); 
    let parent_2 = Genome::new(vec![_4,_2,_5,_1,_6,_8,_3,_7]); 

    let population: Vec<Genome> = vec![parent_1,parent_2];
    Population::new(population,1,1,1,1,2);
}

#[test]
fn new_validation_no_panic(){
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

    let population: Vec<Genome> = vec![parent_1,parent_2];
    Population::new(population,1,1,1,1,2);
}

#[test]
fn test_determine_roulete(){
    let population = given_Population();
    let roulete = determine_roulete(&population.population);
    let counted_result_1: Vec<f64> = vec![8.0/24.0, 18.0/24.0, 24.0/24.0];
    let mut equal = true;
    for i in 0..roulete.len(){
        if !approx_eq!(f64,roulete[i], counted_result_1[i], ulps=4){
            equal = false;
        }
    }
    assert_eq!(equal, true);
}

#[test]

fn test_take_index_from_roulete(){
    let population = given_Population();
    let roulete = determine_roulete(&population.population);
    let index_0 = take_index_from_roulete(&roulete, 0.1);
    let index_1 = take_index_from_roulete(&roulete, 0.4);
    let index_2 = take_index_from_roulete(&roulete, 0.9);
    assert_eq!(index_0,0);
    assert_eq!(index_1,1);
    assert_eq!(index_2,2);
}

#[test]
fn pick_crossovers(){
    let population = given_Population(); 
    for _ in 1..1000{
        let parents = population.pick_crossovers();
        assert_eq!(parents[0].0 == parents[0].1, false);
    }
}

#[test]
fn test_crossover_2ofsprings(){ // the test sometimes will fail becouse of randomistaion. It can create offsprings whcich actualy exist in population, then they are not added, so popolutation stay the same.
    let mut population = given_Population_long(); 
    let len_before = population.population.len();
    population.crossover_2ofsprings();
    
    assert_eq!(len_before < population.population.len(), true);
}

#[test]
fn reduce(){
    let mut population = given_Population(); 
    let len_before = population.population.len();
    population.crossover_2ofsprings();
    population.reduce();
    assert_eq!(len_before,population.population.len());
}

#[test]
fn as_string(){
    let mut population = given_Population();
    assert_eq!(population.as_string(),"Population: \n===========================================================================================================================\nGenome: (1 , 1) | (1 , 4) | (1 , 2) | (1 , 3) | \nGenome: (1 , 4) | (1 , 1) | (1 , 3) | (1 , 1) | \nGenome: (1 , 1) | (1 , 2) | (1 , 3) | (1 , 4) | \n")
}

#[test]
fn as_string_with_fitnes(){
    let mut population = given_Population();
    assert_eq!(population.as_string_with_fitnes(),"Population: \n===========================================================================================================================\nGenome: (1 , 1) | (1 , 4) | (1 , 2) | (1 , 3) | => 8\nGenome: (1 , 4) | (1 , 1) | (1 , 3) | (1 , 1) | => 10\nGenome: (1 , 1) | (1 , 2) | (1 , 3) | (1 , 4) | => 6\n")
}

fn given_Population() -> Population{
    let _1 = Node::new(1.0,1.0);
    let _2 = Node::new(1.0,2.0);
    let _3 = Node::new(1.0,3.0);
    let _4 = Node::new(1.0,4.0);

    let parent_3 = Genome::new(vec![_1,_2,_3,_4]); // total distance: 6     | 6/24  = 0,25
    let parent_2 = Genome::new(vec![_4,_1,_3,_1]); // total distance: 10    | 10/24 = 0,4166
    let parent_1 = Genome::new(vec![_1,_4,_2,_3]); // total distance: 8     | 8/24  = 0,3333
                                                   // sum = 24
                                                    
    let genoms: Vec<Genome> = vec![parent_1,parent_2,parent_3];
    Population::new(genoms, 1, 1, 1, 1, 2)
} 

fn given_Population_long() -> Population{
    let _1 = Node::new(1.0,1.0);
    let _2 = Node::new(1.0,2.0);
    let _3 = Node::new(1.0,3.0);
    let _4 = Node::new(1.0,4.0);
    let _5 = Node::new(1.0,4.0);
    let _6 = Node::new(1.0,4.0);
    let _7 = Node::new(1.0,4.0);
    let _8 = Node::new(1.0,4.0);
    let _9 = Node::new(1.0,4.0);


    let parent_3 = Genome::new(vec![_1,_2,_3,_4,_6,_8,_7,_9,_5]); // total distance: 6     | 6/24  = 0,25
    let parent_2 = Genome::new(vec![_4,_1,_3,_1,_7,_9,_6,_5,_8]); // total distance: 10    | 10/24 = 0,4166
    let parent_1 = Genome::new(vec![_1,_8,_2,_3,_9,_4,_6,_5,_7]); // total distance: 8     | 8/24  = 0,3333
                                                   // sum = 24
                                                    
    let genoms: Vec<Genome> = vec![parent_1,parent_2,parent_3];
    Population::new(genoms, 1, 1, 1, 1, 2)
} 

















