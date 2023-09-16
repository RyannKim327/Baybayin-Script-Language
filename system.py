from builtins import print as ilimbag, input as itala
import os as _____


def __magsalin__():
	reserved_keywords = [
		"and", "except", "lambda", "with", "as", "finally", "nonlocal", "while", "assert", "False", "None", "yield", "break", "for", "not",
		"class", "from", "or", "continue", "global", "pass", "def", "if", "raise", "del", "import", "return", "elif", "in", "True", "else", "is", "try"
	]
	tagalog = [
		"at_saka", "maliban", "lambda", "kasama", "bilang", "sa_wakas", "nonlocal", "habang", "assert", ""
	]
	__result__ = ""
	
	return __result__

def __taga_sala__(file: str):
	extension = ".mag_sawa_ka_na"
	if not file.endswith(extension):
		return f"Hindi ito 'yong file na ginagamit para sa sawa"
	if _____.path.exists(file):
		with open(file, "r") as f1:
			return __magsalin__(f1.read())
	else:
		return f"Hindi ito 'yong file na ginagamit para sa sawa"