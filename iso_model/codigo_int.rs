use aplicacao.rs::apresentacao;

fn main(){
	let frase="A";
	let pal=aplicacao(&frase);
	println!("{}",pal);
	let _oi=receber(&pal);
	println!("{}",_oi);
	dividir(&_oi);
}
fn aplicacao(frase: &str)->String{
	let camada1= format!("{},aplicacao",frase);
	let palavra= apresentacao(&camada1);
	return palavra;
}
fn apresentacao(camada1: &str)->String{
	let camada2= format!("{},apresentacao",camada1);
	let palavra= sessao(&camada2);
	return palavra;
}
fn sessao(camada2: &str)->String{
	let camada3 = format!("{},sessao",camada2);
	let palavra= transporte(&camada3);
	return palavra;
}
fn transporte(camada3: &str)->String{
	let camada4 = format!("{},transporte",camada3);
	let palavra= rede(&camada4);
	return palavra;
}
fn rede(camada4: &str)->String{
	let camada5 = format!("{},rede",camada4);
	let palavra=enlace_de_dados(&camada5);
	return palavra;
}
fn enlace_de_dados(camada5: &str)->String{
	let camada6= format!("{},enlace de dados",camada5);
	let palavra=fisico(&camada6);
	return palavra;
}
fn fisico(camada6: &str)->String{
	let camada7= format!("{},fisico",camada6);
	let mut novafrase= String::from("");
	for l in camada7.chars(){
		let a=format!("{:08b}", l as u8);
		novafrase.push_str(&a);				
	}
	return novafrase;
}

fn receber(novafrase: &str)->String{
	let mut bits = 0;
	let mut byte= String::from("");
	let mut frasenova= String::from("");
	for i in novafrase.chars(){
		bits+=1;	
		byte.push(i);
		if bits ==8{
			bits=0;
			let valor = u8::from_str_radix(&byte, 2).unwrap();
			frasenova.push(valor as char);
			byte.clear();			
		}		
	}
	return frasenova;
}
fn dividir(oi: &str){
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

