#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
	Atom(char),
	Op(char),
	Eof,
}

struct Lexer {
	tokens: Vec<Token>,
}

impl Lexer {
	fn new(input: &str) -> Lexer {
		let tokens: Vec<_> = input
			.chars()
			.filter(|it| !it.is_ascii_whitespace())
			.map(|c| match c {
				'0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
				_ => Token::Op(c),
			})
			.rev()
			.collect();
		Lexer { tokens }
	}
}

#[cfg(test)]
mod tests {
	use crate::*;
	#[test]
	fn lexer_new() {
		use crate::Token::*;
		let case = "1 + 2";
		let expected = Lexer {
			tokens: vec![Atom('2'), Op('+'), Atom('1')],
		};
		assert_eq!(Lexer::new(case).tokens, expected.tokens);
	}
}
