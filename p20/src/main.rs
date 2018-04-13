#![feature(nll)]

use std::fmt;
use std::cmp::Ordering;
use std::ops::Index;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

#[derive(Debug, Copy, Clone)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, PartialEq)]
struct V3 {
    x : i32,
    y : i32,
    z : i32,
}

#[derive(Clone)]
struct Particle {
    num : usize,
    valid : bool,
    p : V3,
    v : V3,
    a : V3,
}

struct ParticleGroup {
    particles : Vec<Particle>,
    old_particles : Vec<Particle>,
}

impl V3 {
    fn from(input : &str) -> V3 {
        lazy_static! {
            static ref RE_V3 : regex::Regex = Regex::new(r"^<(-?\d+),(-?\d+),(-?\d+)>$").expect("failed to compile regex");
        }

        let captures = RE_V3.captures_iter(input).next().unwrap();

        V3 {
            x : captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y : captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            z : captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        }
    }

    fn abs_sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn add_to(&mut self, other : &V3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl fmt::Display for V3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{},{}> ({})", self.x, self.y, self.z, self.abs_sum())
    }
}

impl Index<Axis> for V3 {
    type Output = i32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl Particle {
    fn from(num : usize, input : &str) -> Particle {
        lazy_static! {
            static ref RE_PARTICLE : regex::Regex = Regex::new(r"^p=(.*), v=(.*), a=(.*)$").expect("failed to compile regex");
        }

        let captures = RE_PARTICLE.captures_iter(input).next().unwrap();

        Particle {
            num : num,
            valid : true,
            p : V3::from(captures.get(1).unwrap().as_str()),
            v : V3::from(captures.get(2).unwrap().as_str()),
            a : V3::from(captures.get(3).unwrap().as_str()),
        }
    }

    fn compare_by_reach_unstable(p1 : &Particle, p2 : &Particle) -> Ordering {
        let mut res = p1.a.abs_sum().cmp(&p2.a.abs_sum());
        if res == Ordering::Equal {
            res = p1.v.abs_sum().cmp(&p2.v.abs_sum());

            if res == Ordering::Equal {
                res = p1.p.abs_sum().cmp(&p2.p.abs_sum());

                if res == Ordering::Equal {
                    panic!("two equal particles!");
                }
            }
        }

        res
    }

    fn invalidate(&mut self) {
        self.valid = false;
    }

    fn is_valid(&self) -> bool {
        self.valid
    }

    fn can_intersect_1d(&self, other : &Particle, axis : Axis) -> bool {
        // This function is only accurate if the speed along this axis is known to be increasing;
        // if the speed is decreasing that means the acceleration is pointing in the opposite
        // direction of the velocity, and so this function's comparisons of velocity will be wrong.
        assert!(self.a[axis] == 0 || self.v[axis].cmp(&0) == self.a[axis].cmp(&0));

        // Are we moving towards the other particle's position on this axis?
        other.v[axis] != 0 && self.p[axis].cmp(&other.p[axis]) == other.v[axis].cmp(&0) &&

        // If also heading in different directions, that means they're approaching each other.
        (self.v[axis].cmp(&0) != other.v[axis].cmp(&0) ||

        // If heading in the same direction, then the other particle could intersect this one if
        // it's going faster. I.e. it could catch up.
         other.v[axis].abs() > self.v[axis].abs() ||

        // If the other particle's speed is the same or less, that could change if its acceleration
        // is greater than this one's.
         other.a[axis].abs() > self.a[axis].abs())
    }

    fn can_intersect(&self, other : &Particle) -> bool {
        self.can_intersect_1d(other, Axis::X) ||
        self.can_intersect_1d(other, Axis::Y) ||
        self.can_intersect_1d(other, Axis::Z)
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: p={}, v={}, a={}", self.num, self.p, self.v, self.a)
    }
}

impl ParticleGroup {
    fn new(particles : Vec<Particle>) -> ParticleGroup {
        ParticleGroup {
            particles : particles,
            old_particles : vec![],
        }
    }

    fn sort_by_reach(&mut self) {
        self.particles.sort_unstable_by(Particle::compare_by_reach_unstable);
    }

    // This runs a simulation of the particle movement until it can conclusively tell that it's
    // impossible for any particle to collide anymore. The assumption is that, given enough time,
    // all particles would either collide and be removed from the simulation or be moving away
    // from each other (visualize a spiky ball, where the spikes are directions of particles
    // moving).
    fn next_debug(&mut self, debug : bool) -> Option<usize> {
        // Always run at least one step of the simulation, to give a final answer to the caller
        // even if the very first step knows no collisions are possible.
        let must_ret = self.old_particles.is_empty();
        self.old_particles = self.particles.clone();

        for particle in self.particles.iter_mut() {
            particle.v.add_to(&particle.a);
            particle.p.add_to(&particle.v);
        }

        let mut any_axis_speed_unstable = false;

        for i in 0 .. self.particles.len() {
            let colliding_position = self.particles[i].p.clone();
            let mut found_collision = false;

            // Mark all particles that share the position with colliding_position as invalidated.
            for particle in self.particles.iter_mut().skip(i + 1) {
                if particle.is_valid() && particle.p == colliding_position {
                    eprintln!("collision: {}", particle);
                    found_collision = true;
                    particle.invalidate();
                }
            }

            if found_collision {
                eprintln!("collision: {}", self.particles[i]);
                self.particles[i].invalidate();
            } else {
                if !any_axis_speed_unstable {
                    any_axis_speed_unstable = self.particles[i].v.x.abs() < self.old_particles[i].v.x.abs() ||
                                              self.particles[i].v.y.abs() < self.old_particles[i].v.y.abs() ||
                                              self.particles[i].v.z.abs() < self.old_particles[i].v.z.abs();

                    if debug && any_axis_speed_unstable {
                       eprintln!("slowing down: old: {}, new: {}", self.old_particles[i].v, self.particles[i].v);
                    }
                }
            }
        }

        // Sweep through and remove all invalidated particles.
        self.particles.retain(Particle::is_valid);

        // All velocities are heading in a direction that won't change because acceleration is
        // pointing a different way, so time to check if any particle is on a collision course
        // with any other.
        //
        // This is deliberately very conservative. It only checks if there's a possibility of a
        // future collision. In cases where it can't conclusively tell (such as one particle
        // chasing another one at a greater acceleration or speed), it says a collision is
        // possible. The simulation will run for longer until it can conclusively prove for all
        // particles.
        let no_collisions_possible =
            // If a collision happened in this step of the simulation, run another step.
            self.particles.len() == self.old_particles.len() &&

            // If any particle is decelerating along any axis, then the directionality of the
            // velocity along the axis will eventually change, so defer this check.
            !any_axis_speed_unstable &&

            // Are there no particles...
            !self.particles.iter().any(|particle| {
                // That can collide with any other particle?
                self.particles.iter().any(|other_particle| {
                    let possibly_collides = other_particle.num != particle.num && particle.can_intersect(&other_particle);

                    if debug {
                        if possibly_collides {
                            eprintln!("{} may collide with {}", other_particle, particle);
                        }
                    }

                    possibly_collides
                })
            });

        if !must_ret && no_collisions_possible {
            None
        } else {
            Some(self.particles.len())
        }
    }
}

impl Iterator for ParticleGroup {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_debug(false)
    }
}


fn solve_a(input : &str) -> usize {
    let mut particles = ParticleGroup::new(input.lines().enumerate().map(|(i, line)| {
        Particle::from(i, line)
    }).collect::<Vec<Particle>>());

    particles.sort_by_reach();

    for (i, particle) in particles.particles.iter().enumerate() {
        eprintln!("{}: {}", i, particle);
    }

    particles.particles[0].num
}

fn solve_b(input : &str) -> usize {
    let mut particles = ParticleGroup::new(input.lines().enumerate().map(|(i, line)| {
        Particle::from(i, line)
    }).collect::<Vec<Particle>>());

    let mut iterations = 0;
    let mut final_num_remaining = 0;
    while let Some(num_remaining) = particles.next_debug(iterations % 200 == 0) {
        final_num_remaining = num_remaining;
        if iterations > 0 {
            if iterations % 100 == 0 {
                eprintln!("iter {}: {} left", iterations, particles.particles.len());
            }

            if iterations % 1000 == 0 {
                for (i, particle) in particles.particles.iter().enumerate() {
                    eprintln!("iter {}: {}: {}", iterations, i, particle);
                }
            }
        }

        iterations += 1;
    }

    eprintln!("took {} steps", iterations);
    final_num_remaining
}

fn main() {
    let input = read_all_stdin();
    //eprintln!("input: {}", input);

    if aoclib::should_solve_puzzle_a() {
        println!("answer: {}", solve_a(&input));
    } else {
        println!("answer: {}", solve_b(&input));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_given() {
        let input =
r"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn b_given() {
        let input =
r"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_long() {
        let input =
r"p=<-600,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-400,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-200,0,0>, v=<1,0,0>, a=<0,0,0>
p=<300,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_same_dir_pos_x() {
        let input =
r"p=<0,0,0>, v=<2,0,0>, a=<0,0,0>
p=<100,0,0>, v=<1,0,0>, a=<0,0,0>
p=<-200,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_same_dir_pos_y() {
        let input =
r"p=<0,0,0>, v=<0,2,0>, a=<0,0,0>
p=<0,100,0>, v=<0,1,0>, a=<0,0,0>
p=<-200,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_same_dir_pos_z() {
        let input =
r"p=<0,0,0>, v=<0,0,2>, a=<0,0,0>
p=<0,0,100>, v=<0,0,1>, a=<0,0,0>
p=<-200,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_no_collisions() {
        let input =
r"p=<0,0,0>, v=<0,2,0>, a=<0,0,0>
p=<0,100,0>, v=<1,0,0>, a=<0,0,0>
p=<-200,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 3);
    }

    #[test]
    fn b_decelerating() {
        let input =
r"p=<0,0,0>, v=<0,2,100>, a=<0,0,-1>
p=<0,50,0>, v=<1,0,0>, a=<0,0,0>
p=<-200,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 3);
    }

    #[test]
    fn b_forever_outrun() {
        let input =
r"p=<50,0,0>, v=<2,0,0>, a=<0,0,0>
p=<0,0,0>, v=<1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 2);
    }

    #[test]
    fn b_parallel() {
        let input =
r"p=<1,0,0>, v=<0,1,0>, a=<0,0,0>
p=<0,0,0>, v=<0,1,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 2);
    }

    #[test]
    fn b_forever_chasing() {
        let input =
r"p=<1,0,0>, v=<1,0,0>, a=<0,0,0>
p=<0,0,0>, v=<1,0,0>, a=<0,0,0>";
        assert_eq!(solve_b(&input), 2);
    }
}
