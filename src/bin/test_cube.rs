use vcad::centered_cube;

fn main() {
    let cube = centered_cube("test_cube", 20.0, 20.0, 20.0);
    cube.write_stl("output/test_cube.stl").unwrap();
    println!("Exported: output/test_cube.stl");
}
