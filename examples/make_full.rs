use pba::Model;

fn main() {
    let model = Model::<32>::new(0.0, 0.0, 1.0, None);
    model.write_to_cif("out/mmcif/full.mmcif").unwrap();
}
