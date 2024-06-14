use genome::{fitness_f, Genome};
use population::Population;
use tsp_representation::*;
use itertools::*;
use std::env;
use std::str::FromStr;
use std::time::Instant;
use sysinfo::*;
mod tsp_representation;

fn main() {
    let MiB = (2.0 as f64).powf(20.0);
    let genetic_iteration;

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
        genetic_iteration
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
            parse(&args[9]),
            parse(&args[10])
            ),
        _ => (10,8,0,100,2,2,2,1,3,100000)
    };

    let mut sys = System::new_all();
    sys.refresh_memory();
    let pid = sysinfo::get_current_pid().expect("Failed to get PID");

    //==============================================//
    //              memory measurmet                //
    //==============================================//
    println!("-----------------------------------------------------------");
    println!("measure memory");

    let komiwojazer = Population::new_random(population_len, genome_len, min, max, number_of_crossovers, number_of_mutations, number_of_mutated_genoms, crossover_start, crossover_end);
    // measurment time exactly before brute force problem init.
    // its is important, becouse brute_force_memory(...) shows program memory used in execution
    let mem;
    sys.refresh_memory();
    if let Some(process) = sys.process(pid){
        mem = process.memory();
    }else {
        mem = 0;
    }
    
    println!("\nbrute force");
    println!("memory usage before brute force: {:?}", mem as f64 /MiB);
    let brute_force = komiwojazer.get_best();
    brute_force_memory(brute_force.clone(), &mut sys);


    // measurment time exactly before genetic problem init.
    // after memory measurment the komiwojazer struct is cloned to be included in results.
    // its is important, becouse brute_force_memory(...) shows program memory used in execution
    sys.refresh_memory();
    let komiwojazer_mem = komiwojazer.clone();
    println!("\ngenetic");
    println!("memory usage before genetic: {:?}", mem as f64 /MiB);
    etsp_by_genetic_memory(komiwojazer_mem, genetic_iteration, &mut sys);

    //==============================================//
    //                Time measurmet                //
    //==============================================//
    println!("\n-----------------------------------------------------------");
    println!("measure time");

    println!("\nbrute force");
    etsp_by_brute_force(brute_force);

    println!("\ngenetic");
    etsp_by_genetic(komiwojazer, genetic_iteration)
}

#[allow(dead_code)]
fn etsp_by_genetic_show_populations(mut population: Population, n: u128){
    println!("initial population");
    println!("{}",population.as_string_with_fitnes());

    println!("best genom of initial population: ");
    println!("{}",population.get_best().as_string_with_fitnes());

    let start = Instant::now();
    for _ in 0..n{
        population.round();
    }
    let elapsed = start.elapsed();
    println!("elapsed time: {:?}", elapsed);
    println!("final population");
    println!("{}",population.as_string_with_fitnes());
    
    println!("best genom of final population: ");
    println!("{}",population.get_best().as_string_with_fitnes());
}

fn etsp_by_genetic(mut population: Population, n: u128){
    let start = Instant::now();
    for _ in 0..n{
        population.round();
    }
    let elapsed = start.elapsed();
    println!("elapsed time: , {:?}", elapsed);
   
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

#[allow(non_snake_case)]
fn etsp_by_genetic_memory(mut population: Population, n: u128, sys: &mut System){

    sys.refresh_memory();
    let pid = sysinfo::get_current_pid().expect("Failed to get PID");

    let mut memory_min; 
    let mut memory_max;
    let mut memory_avg;
    let MiB = (2.0 as f64).powf(20.0);
    sys.refresh_memory();
    if let Some(process) = sys.process(pid){
        let mem = process.memory();
        memory_min = mem;
        memory_max = mem;
        memory_avg = 0;
    }else {
        panic!("couldnt read process data");
    }

    for _ in 0..n{
        population.round();

        sys.refresh_memory();
        if let Some(process) = sys.process(pid){
            let mem = process.memory();
            if mem > memory_min{
                memory_min = mem;
            }

            if mem < memory_max{
                memory_max = mem;
            }
            memory_avg += mem;
        }
    }
    println!("memory usage:");
    println!("min: {:?} MiB", (memory_min) as f64 / MiB);
    println!("max: {:?} MiB", (memory_max) as f64 / MiB);
    println!("avg: {:?} MiB", (memory_avg as f64 / (n  as f64) as f64)/ MiB);
}

#[allow(non_snake_case)]
fn brute_force_memory(genome: Genome, sys: &mut System){
    let pid = sysinfo::get_current_pid().expect("Failed to get PID");

    let mut memory_min; 
    let mut memory_max;
    let mut memory_avg;
    let MiB = (2.0 as f64).powf(20.0);
    let len = factorial(genome.get_len());
    
    let g = genome.get_nodes().clone();
    let mut result = genome.fitness_f();
    let mut result_perm = vec![];


    let permutations = g.iter().permutations(g.len());

    sys.refresh_memory();
    if let Some(process) = sys.process(pid){
        let mem = process.memory();
        memory_min = mem;
        memory_max = mem;
        memory_avg = 0;
    }else {
        panic!("couldnt read process data");
    }

    permutations.for_each(|x|{
        if result > fitness_f(x.clone()) {
            result_perm = x.clone();
            result = fitness_f(x);
        }
        sys.refresh_memory();
        if let Some(process) = sys.process(pid){
            let mem = process.memory();
            if mem > memory_min{
                memory_min = mem;
            }

            if mem < memory_max{
                memory_max = mem;
            }
            memory_avg += mem;
        }
    });



    println!("memory usage: ");
    println!("min: {:?} MiB", (memory_min) as f64 / MiB);
    println!("max: {:?} MiB", (memory_max) as f64 / MiB);
    println!("avg: {:?} MiB", (memory_avg) as f64 / len / MiB);
}




fn parse<T: FromStr>(s: &str) -> T{
    match T::from_str(&s){
        Ok(l) => l,
        _ => panic!("error in parse argument: {}", s)
    }
}

fn factorial(n: usize) -> f64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result as f64
}