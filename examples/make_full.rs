use pba::Model;

fn main() {
    let model = Model::<2>::new(0.0, 0.0, 1.0, None);
    model.write_to_cif("cif/full.mmcif").unwrap();
    // model.safe_to_txt("out.txt").unwrap();
}
