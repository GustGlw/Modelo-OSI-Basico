use crate::apresentacao;

pub fn aplicacao(frase: &str)->String{
	let camada1= format!("{},aplicacao",frase);
	let palavra= apresentacao::apresentacao(&camada1);
	return palavra;
}