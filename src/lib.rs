// use vtracer::{Config, Preset, convert_image_to_svg};
// use pyo3::{prelude::*, exceptions};

// #[pyfunction]
// fn run_vtracer(input_fname: &str, output_fname: &str) -> PyResult<()> {
//     println!("Running vtracer for input file {} (writing new file at {})", &input_fname, &output_fname);
//     let args = Config::from_preset(Preset::Photo, &input_fname, &output_fname);

//     match convert_image_to_svg(args) {
//         Ok(_) => Ok(()),
//         Err(e) => Err(exceptions::PyRuntimeError::new_err(format!("Runtime Error: {}", e))),
//     }
// }

// #[pymodule]
// fn rustpy_vtracer(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(run_vtracer, m)?)?;
//     Ok(())
// }
