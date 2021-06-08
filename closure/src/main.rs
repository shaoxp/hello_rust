fn main() {
    let intensity = 10;
    let number = 4;

    generate_workout_plan(intensity, number);
}

use std::thread;
use std::time;
use closure;

fn simulate_expensive_cal(intensity: u32) -> u32{
    println!("calculation very slowly...");
    thread::sleep(time::Duration::from_secs(2));
    intensity
}

fn generate_workout_plan(intensity: u32, random_number: u32){
    let mut expensive_closure = closure::Cacher::new(|num| {
        println!("calculation very slowly...");
        thread::sleep(time::Duration::from_secs(2));
        num
    });
    
    if intensity < 25{
        println!(
            "Today do {} pushhups",
            expensive_closure.value(intensity)
        );

        println!(
            "Next do {} situps!",
            expensive_closure.value(intensity)
        );
    }else{
        if random_number == 3 {
            println!("Take a break today");
        }else{
            println!(
                "Today,run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}

