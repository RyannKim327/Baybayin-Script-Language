from builtins import print as ilimbag, input as itala

reserved_keywords = [
	"and", "except", "lambda", "with", "as", "finally", "nonlocal", "while", "assert", "false", "None", "yield", "break", "for", "not",
	"class", "from", "or", "continue", "global", "pass", "def", "if", "raise", "del", "import", "return", "elif", "in", "True", "else", "is", "try"
]

def scanner(code: str):
	