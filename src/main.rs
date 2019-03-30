extern crate rand;
extern crate ndarray;
use std::{thread, time};

mod vector2;
mod simulation;
mod bug_act;
mod bug;
mod plant;
mod util;
use crate::util::Actor;

fn run_tick(simulation: &mut simulation::Simulation) {
    simulation.tick += 1;

    let mut bugs: Vec<bug::Bug> = simulation.bugs.iter().cloned().collect();
    for bug in &mut bugs {
        bug.act(simulation);
    }

    let mut plants: Vec<plant::Plant> = simulation.plants.iter().cloned().collect();
    for plant in &mut plants {
        plant.act(simulation);
    }

    simulation.bugs = bugs.iter().filter(|b| b.life > 0).map(|b| *b).collect();
    simulation.plants = simulation.plants.iter().filter(|b| b.life > 0).map(|b| *b).collect();
}

fn create_world_string(simulation: &simulation::Simulation) {
    let (width, height) = simulation.size;
    let mut world = ndarray::Array2::<char>::from_elem((width, height), ' ');

    for bug in &simulation.bugs {
        world[[bug.location.0 as usize, bug.location.1 as usize]] = '*';
    }

    for plant in &simulation.plants {
        world[[plant.location.0 as usize, plant.location.1 as usize]] = 'P';
    }

    for row in world.genrows() {
        for ch in row.iter() {
            print!("{}", ch);
        }
        print!("\n");
    }
    println!("--------------")
}

fn main() {
    let mut simulation = simulation::create_simulation();

    loop {
        run_tick(&mut simulation);
        create_world_string(&simulation);
        thread::sleep(time::Duration::from_millis(1000 / 60));
    }
}
