use macroquad::prelude::*;
use ::rand::*;
    
#[derive(Clone)]
struct Bacteria {
    position: Vec2,
    radius: f32,
    genome: Vec<u8>,
    energy: f32,
}

impl Bacteria {
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Bacteria {
            position: vec2(x, y),
            radius,
            genome: vec![0; 100],
            energy: 100.0,
        }
    }

    fn update(&mut self) {
        let mut rng = thread_rng();
        self.position.x += rng.gen_range(-0.5..0.5);
        self.position.y += rng.gen_range(-0.5..0.5);
    }

    fn duplicate(&self) -> Self {
        let mut rng = thread_rng();
        let mut new_bacteria = self.clone();
        for gene in new_bacteria.genome.iter_mut() {
            if rng.gen::<f32>() < 0.01 {
                *gene = 1;
            }
        }
        new_bacteria
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }
}


#[macroquad::main("Bacteria growth")]
async fn main() {
    let mut bacteria = vec![Bacteria::new(50.0, 50.0, 1.0); 1000];
    loop {
        clear_background(BLACK);
        for b in bacteria.iter_mut() {
            b.update();
            b.draw();
            // decrease energy
            b.energy -= 0.1;
            // if energy is high, duplicate
            if b.energy > 90.0 {
                b.duplicate();
                b.energy = 100.0;
            }
        }
        next_frame().await
    }
}
