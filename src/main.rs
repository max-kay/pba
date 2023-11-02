use pba::Model;

fn main() {
    let mut model = Model::<4>::new(0.01, 100.0, 2.0 / 3.0, Some("SeEeD"));
    model.save_cif_file("out.cif").unwrap();
}
