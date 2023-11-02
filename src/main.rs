extern crate piston_window;
use fps_counter::FPSCounter;
use piston_window::*;
use rand::Rng;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const PARTICLE_SIZE: f64 = 10.0;
const PARTICLE_NUMBER: i32 = 250;

const INTERACTION_DISTANCE: f64 = 500.0;
const REPULSION_DISTANCE: f64 = 20.0;
const REPULSION_STRENGTH: f64 = 0.05; 
const DAMPING_FACTOR: f64 = 0.98;
const FORCE_SCALING_FACTOR: f64 = 10.0;

const INTERACTION_MATRIX: [[f64; 6]; 6] = [
    // Red    Blue   Green  Yellow Purple Orange
    [  0.9,  -0.5,   0.3,  -0.7,   0.6,  -0.4], // Red
    [  0.4,   0.8,  -0.8,   0.2,  -0.6,   0.1], // Blue
    [ -0.3,   0.6,   0.7,  -0.5,   0.2,  -0.4], // Green
    [  0.5,  -0.2,   0.4,   0.9,  -0.8,   0.3], // Yellow
    [ -0.6,   0.3,  -0.2,   0.7,   0.8,  -0.9], // Purple
    [  0.2,  -0.4,   0.5,  -0.3,   0.1,   0.9], // Orange
];


#[derive(Clone, Copy)]
enum ParticleClass {
    Red =    0,
    Blue =   1,
    Green =  2,
    Yellow = 3,
    Purple = 4,
    Orange = 5,
}

struct Particle {
    position: [f64; 2],
    velocity: [f64; 2],
    particle_class: ParticleClass,
}

impl Particle {
    fn new(particle_class: ParticleClass) -> Self {
        Particle {
            position: getRandomPosition(SCREEN_WIDTH, SCREEN_HEIGHT),
            velocity: [0.0, 0.0],
            particle_class,
        }
    }

    fn update(&mut self) {

        self.velocity[0] *= DAMPING_FACTOR;
        self.velocity[1] *= DAMPING_FACTOR;

        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];


        if self.position[0] <= 0.0 {
            self.position[0] = 0.0;
            self.velocity[0] = -self.velocity[0];
        }
        if self.position[0] >= (SCREEN_WIDTH as f64 - PARTICLE_SIZE) {
            self.position[0] = SCREEN_WIDTH as f64 - PARTICLE_SIZE;
            self.velocity[0] = -self.velocity[0];
        }
        if self.position[1] <= 0.0 {
            self.position[1] = 0.0;
            self.velocity[1] = -self.velocity[1];
        }
        if self.position[1] >= (SCREEN_HEIGHT as f64 - PARTICLE_SIZE) {
            self.position[1] = SCREEN_HEIGHT as f64 - PARTICLE_SIZE;
            self.velocity[1] = -self.velocity[1];
        }
    }

    fn apply_force(&mut self, force: [f64;2]){
        self.velocity[0] += force[0];
        self.velocity[1] += force[1];
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Particle Life", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut particles: Vec<Particle> = Vec::new();
    let mut fps_counter = FPSCounter::new();

    for _ in 0..PARTICLE_NUMBER {
        let p_class = match rand::thread_rng().gen_range(0..6) {
            0 => ParticleClass::Red,
            1 => ParticleClass::Blue,
            2 => ParticleClass::Green,
            3 => ParticleClass::Yellow,
            4 => ParticleClass::Purple,
            _ => ParticleClass::Orange, // _ handles all other values, but in this case, it'll just be 5
        };
        particles.push(Particle::new(p_class));
    }

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            for particle in &mut particles {
                let color = match particle.particle_class {
                    ParticleClass::Red => [1.0, 0.0, 0.0, 1.0],
                    ParticleClass::Blue => [0.0, 0.0, 1.0, 1.0],
                    ParticleClass::Green => [0.0, 1.0, 0.0, 1.0],
                    ParticleClass::Yellow => [1.0, 1.0, 0.0, 1.0],
                    ParticleClass::Purple => [0.5, 0.0, 0.5, 1.0],
                    ParticleClass::Orange => [1.0, 0.65, 0.0, 1.0],
                };

                ellipse(color, [particle.position[0], particle.position[1], PARTICLE_SIZE, PARTICLE_SIZE], context.transform, graphics);

                particle.update(); // Update particle position
                // Update particle based on interactions...
            }
            let fps = fps_counter.tick();
            println!("{}", fps);
        });

        // Handle the interactions outside the draw loop
        for i in 0..particles.len() {
            for j in (i+1)..particles.len() {
                let (force_i, force_j) = calculate_force(&particles[i], &particles[j]);
                particles[i].apply_force(force_i);
                particles[j].apply_force(force_j);
            }
        }
    }
}

fn getRandomPosition(screen_width: u32, screen_height: u32) -> [f64; 2] {
    let mut rng = rand::thread_rng();
    [rng.gen_range(0.0..screen_width as f64), rng.gen_range(0.0..screen_height as f64)]
}

fn calculate_force(p1: &Particle, p2: &Particle) -> ([f64; 2], [f64; 2]) {
    let dx = p2.position[0] - p1.position[0];
    let dy = p2.position[1] - p1.position[1];
    let distance = (dx * dx + dy * dy).sqrt();

    let normalized_dx = dx / distance;
    let normalized_dy = dy / distance;

    if distance < REPULSION_DISTANCE {
        let force_magnitude_p1 = -REPULSION_STRENGTH / distance * FORCE_SCALING_FACTOR;
        let force_magnitude_p2 = force_magnitude_p1; // If you want different repulsion based on type, adjust here.
        return (
            [force_magnitude_p1 * normalized_dx, force_magnitude_p1 * normalized_dy],
            [-force_magnitude_p2 * normalized_dx, -force_magnitude_p2 * normalized_dy]
        );
    }

    if distance < INTERACTION_DISTANCE {
        let interaction_value_p1_to_p2 = INTERACTION_MATRIX[p1.particle_class as usize][p2.particle_class as usize];
        let interaction_value_p2_to_p1 = INTERACTION_MATRIX[p2.particle_class as usize][p1.particle_class as usize];
        
        let force_magnitude_p1 = interaction_value_p1_to_p2 / (distance * distance) * FORCE_SCALING_FACTOR;
        let force_magnitude_p2 = interaction_value_p2_to_p1 / (distance * distance) * FORCE_SCALING_FACTOR;
        
        return (
            [force_magnitude_p1 * normalized_dx, force_magnitude_p1 * normalized_dy],
            [-force_magnitude_p2 * normalized_dx, -force_magnitude_p2 * normalized_dy]
        );
    }

    return ([0.0, 0.0], [0.0, 0.0]);
}
