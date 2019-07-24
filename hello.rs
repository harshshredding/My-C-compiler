use std::fs;

#[derive(Debug)]
enum Token {
	OpenBrace,
	CloseBrace,
	OpenPar,
	ClosePar,
	Semicolon,
	IntegerKey,
	ReturnKey,
	Identifier(String),
	Literal(i32)
}

fn compute_token(word: &str) -> Token {
	let count = word.chars().count();
	let chars = word.chars();
	if count == 1 {
		//let charVec = chars.collect();
		for ch in chars {
			if ch.is_digit(10) {
				return Token::Literal(ch.to_digit(10).unwrap() as i32);
			}
			return match ch {
				'{' => Token::OpenBrace,
				'}' => Token::CloseBrace,
				'(' => Token::OpenPar,
				')' => Token::ClosePar,
				';' => Token::Semicolon,
				 _  => Token::Identifier(word.to_string()),
			}
		}
	} else if count > 0 {
		if word == "return" {
			return Token::ReturnKey;
		}
		if word == "int" {
			return Token::IntegerKey;
		}
		let integer = word.trim().parse::<i32>();
		if integer.is_ok() {
			return Token::Literal(integer.unwrap());
		}
		return Token::Identifier(word.to_string());
	}
	return Token::Identifier("".to_string());
}

fn main() {
	let mut vec = Vec::<Token>::new();
	let filename = "return_2.c";
	println!("In file {}", filename);
	let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");
	let lines = contents.lines();
	for line in lines {
		let words = line.split_whitespace();
		for word in words {
			vec.push(compute_token(word));
		}
	}
	println!("{:?}", vec);
}