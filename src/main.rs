use genome::{crossover_2ofsprings, fitness_f, Genome};
use population::Population;
use tsp_representation::*;
use itertools::*;

mod tsp_representation;

fn main() {
    let population_len = 20;
    let genome_len = 10;

    let min = 0;
    let max = 100;

    let number_of_crossovers = 2;
    let number_of_mutations = 2;
    let number_of_mutated_genoms = 2;
    let crossover_start = 3;
    let crossover_end = 6;

    let komiwojazer = Population::new_random(population_len, genome_len, min, max, number_of_crossovers, number_of_mutations, number_of_mutated_genoms, crossover_start, crossover_end);
    let brute_force = komiwojazer.get_best();


    let n: u128 = 2;
    println!("genetycznie");
    ETSP_by_iteration(komiwojazer, 100000);

    println!("=======================================");
    println!("\n\nbrute force");
    ETSP_by_brute_force(brute_force);
}


fn ETSP_by_iteration(mut population: Population, n: u128){
    println!("populacja poczatkowa");
    println!("{}",population.as_string_with_fitnes());

    println!("najlepszy znaleziony genom: ");
    println!("{}",population.get_best().as_string_with_fitnes());
    for i in 0..n{
        population.round();
        if i % 10000 == 0{
            println!("i = {:?}", i);
        }
    }
    println!("populacja koncowa");
    println!("{}",population.as_string_with_fitnes());
    
    println!("najlepszy znaleziony genom: ");
    println!("{}",population.get_best().as_string_with_fitnes());
}

fn ETSP_by_brute_force(genome: Genome){
    let mut g = genome.get_nodes().clone();
    let mut result = genome.fitness_f();
    let mut result_perm = vec![];

    //println!("przed ");
    //println!("{:?}", g);
    //println!("{:?}", result);

    let permutations = g.iter().permutations(g.len());
    permutations.for_each(|x|{
        if result > fitness_f(x.clone()) {
            result_perm = x.clone();
            result = fitness_f(x);
        }
    });

    println!("po ");
    println!("{:?}", result_perm);
    println!("{:?}", result);
}