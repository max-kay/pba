use pba::Model;

fn main() {
    let model = Model::<4>::new(0.0, 0.0, 1.0, None);
    model.write_to_cif("mmcif/full.mmcif").unwrap();
}
