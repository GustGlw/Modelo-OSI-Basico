pub fn dividir(oi: &str){
	let vetor: Vec<&str>=oi.split(',').collect();
	let mensagem_recebida=vetor[0];
	let aplicacao=vetor[1];
	let apresentacao=vetor[2];
	let sessao=vetor[3];
	let transporte=vetor[4];
	let rede=vetor[5];
	let enlace_de_dados=vetor[6];
	let fisico=vetor[7];
	println!("{}/mensagem   	{}/aplicacao	{}/apresentacao 	{}/sessao	{}/transporte   	{}/rede 	{}/enlace_de_dados  	{}/fisico",mensagem_recebida,aplicacao,apresentacao,sessao,transporte,rede,enlace_de_dados,fisico);
}

