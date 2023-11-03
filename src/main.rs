use pba::Model;
const SIZE: usize = 4;
const EPOCH: usize = 10;
fn main() {
    let mut model = Model::<SIZE>::new(0.01, 100.0, 2.0 / 3.0, Some("SeEeD"));
    let tot_steps = EPOCH * SIZE * SIZE * SIZE / 2;
    for _ in 0..tot_steps {
        model.monte_carlo_step(1.0)
    }
    model.write_to_cif("cif_files/out.cif").unwrap();
}
