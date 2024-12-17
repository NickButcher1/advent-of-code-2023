use aoc::solution::{Solution, Solutions};
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Particle {
    px: isize,
    py: isize,
    pz: isize,
    vx: isize,
    vy: isize,
    vz: isize,
    ax: isize,
    ay: isize,
    az: isize,
}

pub fn solve20(input: &[String]) -> Solutions {
    let mut particles = read_particles(input);

    (
        Solution::USIZE(solve_part_1(&mut particles.clone())),
        Solution::USIZE(solve_part_2(&mut particles)),
    )
}

fn read_particles(input: &[String]) -> Vec<Particle> {
    let re = Regex::new(
        r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$",
    )
    .unwrap();

    input
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Particle {
                px: captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                py: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                pz: captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                vx: captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                vy: captures.get(5).unwrap().as_str().parse::<isize>().unwrap(),
                vz: captures.get(6).unwrap().as_str().parse::<isize>().unwrap(),
                ax: captures.get(7).unwrap().as_str().parse::<isize>().unwrap(),
                ay: captures.get(8).unwrap().as_str().parse::<isize>().unwrap(),
                az: captures.get(9).unwrap().as_str().parse::<isize>().unwrap(),
            }
        })
        .collect()
}

// Loop until we are confident that a specific particle remains closest to the origin.
fn solve_part_1(particles: &mut [Particle]) -> usize {
    // TODO This is enough loops but could fix to stop when no particles are still approaching origin.
    for _ in 0..1000 {
        find_closest_to_origin(particles);
        move_particles_one_tick(particles);
    }
    find_closest_to_origin(particles)
}

// Loop until we are confident that no further collisions will happen.
fn solve_part_2(particles: &mut Vec<Particle>) -> usize {
    // TODO This is enough loops, but could fix to stop when all particles are moving away from each other.
    for _ in 0..1000 {
        remove_overlapping_particles(particles);
        move_particles_one_tick(particles);
    }
    particles.len()
}

fn move_particles_one_tick(particles: &mut [Particle]) {
    particles.iter_mut().for_each(|particle| {
        particle.vx += particle.ax;
        particle.vy += particle.ay;
        particle.vz += particle.az;
        particle.px += particle.vx;
        particle.py += particle.vy;
        particle.pz += particle.vz;
    });
}

fn find_closest_to_origin(particles: &[Particle]) -> usize {
    let (closest_idx, _) = particles.iter().enumerate().fold(
        (0, usize::MAX),
        |(closest_idx, closest_taxicab_distance), (idx, particle)| {
            let taxicab_distance = taxicab_distance(particle);
            if taxicab_distance < closest_taxicab_distance {
                (idx, taxicab_distance)
            } else {
                (closest_idx, closest_taxicab_distance)
            }
        },
    );
    closest_idx
}

fn taxicab_distance(particle: &Particle) -> usize {
    (particle.px).abs_diff(0) + (particle.py).abs_diff(0) + (particle.pz).abs_diff(0)
}

fn remove_overlapping_particles(particles: &mut Vec<Particle>) {
    let mut remove_indices: HashSet<usize> = HashSet::new();
    for i in 0..particles.len() {
        for j in 1..particles.len() {
            if j > i
                && particles[i].px == particles[j].px
                && particles[i].py == particles[j].py
                && particles[i].pz == particles[j].pz
            {
                remove_indices.insert(i);
                remove_indices.insert(j);
            }
        }
    }
    if !remove_indices.is_empty() {
        let mut remove_indices_vec: Vec<&usize> = remove_indices.iter().collect();
        remove_indices_vec.sort();
        remove_indices_vec.reverse();
        for i in remove_indices_vec {
            particles.remove(*i);
        }
    }
}
