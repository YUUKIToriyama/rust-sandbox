pub fn han2zen(str: String) -> String {
	return str
		.as_str()
		.chars()
		.map(|c| match c {
			'ｦ' => 'ヲ',
			'ｧ' => 'ァ',
			'ｨ' => 'ィ',
			'ｩ' => 'ゥ',
			'ｪ' => 'ェ',
			'ｫ' => 'ォ',
			'ｬ' => 'ャ',
			'ｭ' => 'ュ',
			'ｮ' => 'ョ',
			'ｯ' => 'ッ',
			'ｰ' => 'ー',
			'ｱ' => 'ア',
			'ｲ' => 'イ',
			'ｳ' => 'ウ',
			'ｴ' => 'エ',
			'ｵ' => 'オ',
			'ｶ' => 'カ',
			'ｷ' => 'キ',
			'ｸ' => 'ク',
			'ｹ' => 'ケ',
			'ｺ' => 'コ',
			'ｻ' => 'サ',
			'ｼ' => 'シ',
			'ｽ' => 'ス',
			'ｾ' => 'セ',
			'ｿ' => 'ソ',
			'ﾀ' => 'タ',
			'ﾁ' => 'チ',
			'ﾂ' => 'ツ',
			'ﾃ' => 'テ',
			'ﾄ' => 'ト',
			'ﾅ' => 'ナ',
			'ﾆ' => 'ニ',
			'ﾇ' => 'ヌ',
			'ﾈ' => 'ネ',
			'ﾉ' => 'ノ',
			'ﾊ' => 'ハ',
			'ﾋ' => 'ヒ',
			'ﾌ' => 'フ',
			'ﾍ' => 'ヘ',
			'ﾎ' => 'ホ',
			'ﾏ' => 'マ',
			'ﾐ' => 'ミ',
			'ﾑ' => 'ム',
			'ﾒ' => 'メ',
			'ﾓ' => 'モ',
			'ﾔ' => 'ヤ',
			'ﾕ' => 'ユ',
			'ﾖ' => 'ヨ',
			'ﾗ' => 'ラ',
			'ﾘ' => 'リ',
			'ﾙ' => 'ル',
			'ﾚ' => 'レ',
			'ﾛ' => 'ロ',
			'ﾜ' => 'ワ',
			'ﾝ' => 'ン',
			'ﾞ' => '゛',
			'ﾟ' => '゜',
			_ => c,
		})
		.collect();
}
