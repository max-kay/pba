use pba::Model;
const SIZE: usize = 8;
const EPOCH: usize = 10;

fn main() {
    let mut model = Model::<SIZE>::new(0.1, 0.1, 2.0 / 3.0, Some("SeEeD"));

    for _ in 0..EPOCH {
        for _ in 0..SIZE * SIZE * SIZE {
            model.monte_carlo_step(1.0 / 100.0)
        }
        // model.print_counters();
        model.print_hamiltonian()
    }
    println!("finished");
    model.print_hamiltonian();
    model.print_neighbours();
    println!("");
    println!("new hamiltonian {}", model.calc_hamiltonian());
    model.print_neighbours();

    model.write_to_cif("cif_files/out.cif").unwrap();
}
