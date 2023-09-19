mod models;
use models::tuple::*;
use models::projectile::*;
use rust_decimal_macros::dec;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut projectile = Projectile { position: point(0.0, 1.0, 0.0), velocity: vector(1.0, 1.0, 0.0).normalise() };
    let environment = Environment { gravity: vector(0.0, -0.1, 0.0), wind: vector(-0.01, 0.0, 0.0) };
    let mut counter = 0;

    println!("starting {}", projectile.position);

    projectile = tick(&environment, &projectile);

    loop {
        projectile = tick(&environment, &projectile);

        println!("");

        println!("dropping to {}", projectile.position);

        if projectile.position.y <= dec!(0.0) {
            println!("we hit the ground at {}", projectile.position);
            break;
        }

        counter += 1;

        if counter == 100 {
            break;
        }

        sleep(Duration::from_secs(1));
    }
}
