mod aplicacao;
mod apresentacao;
mod fisico;
mod rede;
mod sessao;
mod transporte;
mod enlace_de_dados;
mod dividir;
mod receber;



fn main(){
	let frase="A";
	let pal=aplicacao::aplicacao(&frase);
	println!("{}",pal);
	let _oi=receber::receber(&pal);
	println!("{}",_oi);
	dividir::dividir(&_oi);
}



