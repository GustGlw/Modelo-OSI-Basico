pub fn fisico(camada6: &str)->String{
	let camada7= format!("{},fisico",camada6);
	let mut novafrase= String::from("");
	for l in camada7.chars(){
		let a=format!("{:08b}", l as u8);
		novafrase.push_str(&a);				
	}
	return novafrase;
}