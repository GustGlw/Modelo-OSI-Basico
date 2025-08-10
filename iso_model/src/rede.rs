use crate::enlace_de_dados;
pub fn rede(camada4: &str)->String{
	let camada5 = format!("{},rede",camada4);
	let palavra=enlace_de_dados::enlace_de_dados(&camada5);
	return palavra;
}