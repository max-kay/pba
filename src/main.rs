use std::time::Instant;

use chrono::Utc;
use rayon::prelude::*;

use pba::{CsvLogger, Model, StreamingStats};
const J_2: f32 = 1.0;

const SIZE: usize = 32;
const FILL_FRAC: f32 = 2.0 / 3.0;

const EQ_EPOCHS: usize = 500;
const EPOCH: usize = 500;

const J_STEPS: u32 = 16;
const J_START: f32 = 6.0;
const J_END: f32 = 0.0;

const TEMP_STEPS: u32 = 40;
const LN_T_PRIME_0: f32 = 4.0;
const LN_T_PRIME_END: f32 = -2.0;

fn main() {
    let name = format!("{}", Utc::now().format("%Y-%m-%d_%H-%M"));
    std::fs::create_dir(&format!("out/mmcif/{}", name)).unwrap();
    std::fs::create_dir(&format!("out/models/{}", name)).unwrap();

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
        format!("out/csv/{}.csv", name),
        format!(
            "energy and variance are give per cyanometalate site\n{} supercells in every direction\n{} fill fraction",
            SIZE/2,
            (FILL_FRAC*(SIZE*SIZE*SIZE/2) as f32).floor() as usize as f32 / (SIZE*SIZE*SIZE/2) as f32
        ),
        vec!["j_prime", "temp", "energy", "variance"],
    );

    let start = Instant::now();
    let _: Vec<_> = j_primes
        .par_iter()
        .map_with(logger, |logger, j_prime| {
            let mut model = Model::<SIZE>::new(j_prime * J_2, J_2, FILL_FRAC, None);
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
                if let Result::Err(err) = model.write_to_cif(&format!(
                    "out/mmcif/{}/j_{}_t_{}.mmcif",
                    name, j_prime, temp
                )) {
                    eprintln!(
                        "{}\ncould not create mmcif for j: {}, t: {}",
                        err, j_prime, temp
                    )
                }

                if let Result::Err(err) =
                    model.safe_to_txt(&format!("out/models/{}/j_{}_t_{}.txt", name, j_prime, temp))
                {
                    eprintln!(
                        "{}\ncould not create txt for j: {}, t: {}",
                        err, j_prime, temp
                    )
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
