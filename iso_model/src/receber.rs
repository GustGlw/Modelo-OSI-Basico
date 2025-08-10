
pub fn receber(novafrase: &str)->String{
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