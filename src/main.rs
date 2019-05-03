extern crate rand;
extern crate ndarray;

use std::{thread, time};
mod vector2;
mod simulation;
mod bug_act;
mod plant_act;
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
    simulation.bugs = bugs.iter().filter(|b| b.life > 0).map(|b| *b).collect();

    let mut plants: Vec<plant::Plant> = simulation.plants.iter().cloned().collect();
    for plant in &mut plants {
        plant.act(simulation);
    }
    simulation.plants = plants.iter().filter(|b| b.life > 0).map(|b| *b).collect();

    let mut seeds: Vec<plant::Seed> = simulation.seeds.iter().cloned().collect();
    for seed in &mut seeds {
        seed.act(simulation);
    }
    simulation.seeds = seeds.iter().filter(|b| !b.seeded).map(|b| *b).collect();

    let mut eggs: Vec<bug::Egg> = simulation.eggs.iter().cloned().collect();
    for egg in &mut eggs {
        egg.act(simulation);
    }
    simulation.eggs = eggs.iter().filter(|b| b.time_until_hatch > 0).map(|b| *b).collect();
}

fn create_world_string(simulation: &simulation::Simulation) {
    let (width, height) = simulation.size;
    let mut world = ndarray::Array2::<String>::from_elem((width, height), "  ".to_string());

    for egg in &simulation.eggs {
        world[[egg.location.0 as usize, egg.location.1 as usize]] = "🥚".to_string();
    }

    for bug in &simulation.bugs {
        world[[bug.location.0 as usize, bug.location.1 as usize]] = "🐞".to_string();
    }

    for seed in &simulation.seeds {
        world[[seed.location.0 as usize, seed.location.1 as usize]] = "🌱".to_string();
    }

    for plant in &simulation.plants {
        world[[plant.location.0 as usize, plant.location.1 as usize]] = "🌳".to_string();
    }

    let mut world_stirng = String::from("");

    for row in world.genrows() {
        for ch in row.iter() {
            world_stirng = world_stirng + ch;
        }
        world_stirng = world_stirng + "\n";
    }

    print!("{}", world_stirng);

}

fn main() {
    let mut simulation = simulation::create_simulation();

    let mut tick = 0;
    loop {
        print!("{}[2J", 27 as char);
        tick += 1;
        if tick > 10000 {
            break;
        }
        run_tick(&mut simulation);
        if simulation::is_dead_world(&simulation) {
            break;
        }

        create_world_string(&simulation);

        thread::sleep(time::Duration::from_millis(1000 / 15));
    }
}
