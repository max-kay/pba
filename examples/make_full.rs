use pba::Model;

fn main() {
    let model = Model::<10>::new(0.0, 0.0, 1.0 / 3.0, None);
    model.write_to_cif("cif/filled.cif").unwrap();
    model.safe_to_txt("out.txt").unwrap();
}
