pub struct RpnCalculator(bool);

impl RpnCalculator {
	pub fn new(verbose: bool) -> Self {
		return Self(verbose);
	}
	pub fn eval(&self, formula: &str) -> i32 {
		let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
		return self.eval_inner(&mut tokens);
	}
	fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
		let mut stack = Vec::new(); // 数字を取り出してはスタックに収納していく
		while let Some(token) = tokens.pop() {
			if let Ok(x) = token.parse::<i32>() {
				// 取り出したトークンの型がi32の場合スタックに収納
				stack.push(x);
			} else {
				// 演算子である場合、スタックから値を取り出して演算
				// 値を取り出せない場合、panicを起こして終了
				let y = stack.pop().expect("invalid syntax");
				let x = stack.pop().expect("invalid syntax");
				let result = match token {
					"+" => x + y,
					"-" => x - y,
					"*" => x * y,
					"/" => x / y,
					"%" => x % y,
					_ => panic!("invalid token"),
				};
				stack.push(result);
			}
			// verboseフラグが立っている場合は計算過程を表示
			if self.0 {
				println!("{:?} {:?}", tokens, stack);
			}
		}
		if stack.len() == 1 {
			return stack[0];
		} else {
			panic!("invalid syntax");
		}
	}
}
