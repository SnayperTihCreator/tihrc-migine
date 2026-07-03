use pyo3_stub_gen::Result;
use _core_migine;

fn main() -> Result<()>{
    let stub = _core_migine::stub_info()?;
    stub.generate()?;
    Ok(())
}