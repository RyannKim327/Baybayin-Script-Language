reserved_keywords = [
	"and", "except", "lambda", "with", "as", "finally", "nonlocal", "while", "assert", "false", "None", "yield", "break", "for", "not",
	"class", "from", "or", "continue", "global", "pass", "def", "if", "raise", "del", "import", "return", "elif", "in", "True", "else", "is", "try"
]

def test(text, dulo="\n", pagitan=" ", file=None, flush=False):
	return f"print({test}, end='{dulo}', sep='{pagitan}', file='{file}', flush={flush})"

print("test('hahaha')")
