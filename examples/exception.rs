
extern crate julia;

use julia::error::Result;
use julia::api::{Julia, Ref, FromJulia};

fn func() -> Result<Vec<Ref>> {
    let mut jl = Julia::new();

    let mut results = vec![];
    // a couple exceptions that can occur in Julia
    results.push(jl.eval_string("sqrt(4.0)")?);
    results.push(jl.eval_string("sqrt(-1.0)")?);
    results.push(jl.eval_string("x")?);
    results.push(jl.eval_string("[][1]")?);

    Ok(results)
}

fn main() {
    match func() {
        Ok(vals) => {
            println!("Everything went fine!");
            for val in vals {
                println!("{}", f64::from_julia(&val).unwrap());
            }
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
