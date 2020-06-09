use rand::prelude::*;
use rand::distributions::Uniform;
use std::collections::HashMap;

fn main() {

	let n = 1_000;
    
	let uniforme = Uniform::new_inclusive(1,100);

	let mut numeros : Vec<u64> = Vec::new();

	for _ in 0..n {
		numeros.push(thread_rng().sample(uniforme));
		//numeros.push(normal.sample(&mut thread_rng()));
	}

	numeros.sort();

	let mediana: f64;

	if n % 2 == 0 {
		let ind = ((n as f64)/2.0) as usize;
		mediana = (&numeros[ind-1] + &numeros[ind]) as f64/2.0;
	} else {
		let ind = ((n as f64)/2.0).floor() as usize;
		mediana = *&numeros[ind] as f64;
	}

	let mut contador = HashMap::new();

	let mut suma = 0;

	for numero in &numeros {
		suma += numero;
		let count = contador.entry(numero).or_insert(0);
		*count += 1;
	}

	let mut maximo = 0;
	let mut moda = 0;

	for (key, value) in &contador {
		if *value >= maximo {
			println!("{}: {}",key, value);
			maximo = *value;
			moda = **key;
		}
	}

	let promedio = (suma as f64)/(n as f64);

	println!("promedio: {}", promedio);
	println!("mediana: {}",mediana);
	println!("moda: {}", moda);

}
