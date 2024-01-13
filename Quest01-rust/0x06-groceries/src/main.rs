
pub fn insert(vec: &mut Vec<String>, val: String) {
	vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
	let index2: &String = &vec[index];	
	index2.to_string()
}

fn main() {
	let mut groceries = vec![
		"yogurt".to_string(),
		"panettone".to_string(),
		"bread".to_string(),
		"cheese".to_string(),
	];
	insert(&mut groceries, String::from("nuts"));
	println!("The groceries list contains {:?}", &groceries);
	println!(
		"The second element of the grocery  list is {:?}",
		at_index(&groceries, 1)
	);
}

