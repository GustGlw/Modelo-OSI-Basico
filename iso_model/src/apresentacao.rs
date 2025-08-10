use crate::sessao;
pub fn apresentacao(camada1: &str)->String{
	let camada2= format!("{},apresentacao",camada1);
	let palavra= sessao::sessao(&camada2);
	return palavra;
}