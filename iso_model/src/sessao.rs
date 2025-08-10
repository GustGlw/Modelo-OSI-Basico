use crate::transporte;
pub fn sessao(camada2: &str)->String{
	let camada3 = format!("{},sessao",camada2);
	let palavra= transporte::transporte(&camada3);
	return palavra;
}