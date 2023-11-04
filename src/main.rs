use std::time::Instant;

use chrono::Utc;
use rayon::prelude::*;

use pba::{CsvLogger, Model, StreamingStats};
const J_2: f32 = 1.0;

const SIZE: usize = 64;

const EQ_EPOCHS: usize = 200;
const EPOCH: usize = 300;

const J_STEPS: u32 = 40;
const J_START: f32 = 5.0;
const J_END: f32 = -1.0;

const TEMP_STEPS: u32 = 40;
const LN_T_PRIME_0: f32 = 4.0;
const LN_T_PRIME_END: f32 = 0.0;

fn main() {
    let name = format!("{}", Utc::now().format("%Y-%m-%d_%H-%M"));
    let temps: Vec<f32> = (0..TEMP_STEPS)
        .map(|i| {
            ((LN_T_PRIME_END - LN_T_PRIME_0) / (TEMP_STEPS - 1) as f32 * i as f32 + LN_T_PRIME_0)
                .exp()
                * J_2
        })
        .collect();

    let j_primes: Vec<f32> = (0..J_STEPS)
        .map(|i| i as f32 / (J_STEPS - 1) as f32 * (J_END - J_START) + J_START)
        .collect();

    let (logger, handle) = CsvLogger::new(
        format!("csv/{}.csv", name),
        format!(
            "energy and variance are give per cyanometalate site\n the system size was {}",
            SIZE
        ),
        vec!["j_prime", "temp", "energy", "variance"],
    );

    let start = Instant::now();
    let _: Vec<_> = j_primes
        .par_iter()
        .map_with(logger, |logger, j_prime| {
            let mut model = Model::<SIZE>::new(j_prime * J_2, J_2, 2.0 / 3.0, None);
            for temp in &temps {
                for _ in 0..EQ_EPOCHS {
                    for _ in 0..SIZE * SIZE * SIZE {
                        model.monte_carlo_step(1.0 / temp)
                    }
                }

                let mut stats = StreamingStats::new();
                for _ in 0..EPOCH {
                    for _ in 0..SIZE * SIZE * SIZE {
                        model.monte_carlo_step(1.0 / temp);
                        stats.add_value(model.get_hamiltonian())
                    }
                }
                logger
                    .send_row(vec![
                        *j_prime,
                        *temp,
                        stats.avg() / (SIZE * SIZE * SIZE / 2) as f32,
                        stats.variance() / (SIZE * SIZE * SIZE / 2) as f32,
                    ])
                    .expect("error while sending row to csv logger");
                if let Result::Err(_) =
                    model.write_to_cif(&format!("cif/{}_j_{}_t_{}.cif", name, j_prime, temp))
                {
                    eprintln!("could not create cif for j: {}, t: {}", j_prime, temp)
                }
            }
        })
        .collect();
    handle
        .join()
        .unwrap()
        .expect("error while joining handle of csv logger");
    println!("finished in {}s", start.elapsed().as_secs_f32())
}
