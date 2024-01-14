
pub fn insert(vec: &mut Vec<String>, val: String) {
	vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
	let index2: &String = &vec[index];	
	index2.to_string()
}
