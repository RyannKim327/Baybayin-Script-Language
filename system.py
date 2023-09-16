from builtins import print as ilimbag, input as itala
import os as _____, re as __regex__


def __magsalin__(__code__: str):
	reserved_keywords = [
		"and", "except", "lambda", "with", "as", "finally", "nonlocal", "while", "assert", "False", "None", "yield", "break", "for", "not",
		"class", "from", "or", "continue", "global", "pass", "def", "if", "raise", "del", "import", "return", "elif", "in", "True", "else", "is", "try"
	]
	tagalog = [
		"at_saka", "maliban", "lambda", "kasama", "bilang", "sa_wakas", "nonlocal", "habang", "assert", "mali", "wala", "yield", "hinto", "para_sa", "hindi",
		"class", "mula", "o", "sa_pagpapatuloy", "pangkalahatan", "pass", "def", "kung", "itaas", "burahin", "ibilang", "ibalik", "o_kung_kaya", "sa", "tama", "kung_wala", "is", "subok"
	]
	__result__ = ""
	__data__ = __code__.split("\n")
	for __d__ in __data__:
		__data2__ = __d__.split(" ")
		for __d2__ in __data2__:
			# print(__d2__.replace(":", ""))
			if __d2__.replace(":", "") in tagalog:
				for i in range(len(tagalog)):
					if tagalog[i] == __d2__.replace(":", ""):
						__result__ += reserved_keywords[i]
			else:
				__result__ += __d2__
			if __d2__.endswith(":"):
				__result__ += ":"
			__result__ += " "
		__result__ += "\n"
	return __result__

def __taga_sala__(file: str):
	extension = ".baybay"
	if not file.endswith(extension):
		return f"Hindi ito 'yong file na ginagamit para sa sawa"
	if _____.path.exists(file):
		with open(file, "r") as f1:
			return __magsalin__(f1.read())
	else:
		return f"Hindi ito 'yong file na ginagamit para sa sawa"

a = __magsalin__("""
kung tama:
	ilimbag("tama")
""")
print(a)