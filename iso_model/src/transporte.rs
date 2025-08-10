use crate::rede;
pub fn transporte(camada3: &str)->String{
	let camada4 = format!("{},transporte",camada3);
	let palavra= rede::rede(&camada4);
	return palavra;
}