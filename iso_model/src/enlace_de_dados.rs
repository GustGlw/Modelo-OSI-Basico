use crate::fisico;
pub fn enlace_de_dados(camada5: &str)->String{
	let camada6= format!("{},enlace de dados",camada5);
	let palavra=fisico::fisico(&camada6);
	return palavra;
}