use genome::{fitness_f, Genome};
use population::Population;
use tsp_representation::*;
use itertools::*;
use std::env;
use std::str::FromStr;
use std::time::Instant;

mod tsp_representation;

fn main() {
    let population_len;
    let genome_len;
    let min;
    let max;
    let number_of_crossovers;
    let number_of_mutations;
    let number_of_mutated_genoms;
    let crossover_start;
    let crossover_end;

    let args: Vec<String> = env::args().collect();
    (
        population_len,
        genome_len,
        min,
        max,
        number_of_crossovers,
        number_of_mutations,
        number_of_mutated_genoms,
        crossover_start,
        crossover_end,
    ) = match args.len() {
        10 =>
            (
            parse(&args[1]),
            parse(&args[2]),
            parse(&args[3]),
            parse(&args[4]),
            parse(&args[5]),
            parse(&args[6]),
            parse(&args[7]),
            parse(&args[8]),
            parse(&args[9])
            ),
        _ => (20,10,0,100,2,2,2,3,6)
    };

    let komiwojazer = Population::new_random(population_len, genome_len, min, max, number_of_crossovers, number_of_mutations, number_of_mutated_genoms, crossover_start, crossover_end);
    let brute_force = komiwojazer.get_best();

    println!("genetycznie");
    etsp_by_genetic_algoritm(komiwojazer, 100000);

    println!("=======================================");
    println!("\n\nbrute force");
    etsp_by_brute_force(brute_force);

}


fn etsp_by_genetic_algoritm(mut population: Population, n: u128){
    println!("initial population");
    println!("{}",population.as_string_with_fitnes());

    println!("best genom of initial population: ");
    println!("{}",population.get_best().as_string_with_fitnes());

    let start = Instant::now();
    for _ in 0..n{
        population.round();
    }
    let elapsed = start.elapsed();
    println!("elapsed time: , {:?}", elapsed);
    println!("final population");
    println!("{}",population.as_string_with_fitnes());
    
    println!("best genom of final population: ");
    println!("{}",population.get_best().as_string_with_fitnes());
}

fn etsp_by_brute_force(genome: Genome){
    let g = genome.get_nodes().clone();
    let mut result = genome.fitness_f();
    let mut result_perm = vec![];


    let permutations = g.iter().permutations(g.len());
    let start = Instant::now();
    permutations.for_each(|x|{
        if result > fitness_f(x.clone()) {
            result_perm = x.clone();
            result = fitness_f(x);
        }
    });
    let elapsed = start.elapsed();
    println!("elapsed time: {:?} ", elapsed);
    println!("genome: {:?}", result_perm);
    println!("length: {:?}", result);
}


fn parse<T: FromStr>(s: &str) -> T{
    match T::from_str(&s){
        Ok(l) => l,
        _ => panic!("error in parse argument: {}", s)
    }
}