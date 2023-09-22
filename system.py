from builtins import print as ilimbag, input as itala
import os as _____, re as __regex__

class system:
	def __init__(self, file: str):
		extension = ".pyby"
		if not file.endswith(extension):
			self.file = file + extension
			ilimbag(f"Hindi ito 'yong file na ginagamit para sa sawa")
		if _____.path.exists(file):
			self.file = file
		else:
			self.file = ""
			ilimbag(f"Hindi ito 'yong file na ginagamit para sa sawa")
	
	def __magsalin__(self, __code__: str):
		reserved_keywords = [
			"and", "except", "lambda", "with", "as", "finally", "nonlocal", "while", "assert", "False", "None", "yield", "break", "for", "not",
			"class", "from", "or", "continue", "global", "pass", "def", "if", "raise", "del", "import", "return", "elif", "in", "True", "else", "is", "try"
		]
		tagalog = [
			"at", "maliban", "lambda", "kasama", "bilang", "sa_wakas", "nonlocal", "habang", "assert", "mali", "wala", "yield", "hinto", "para_sa", "hindi",
			"class", "mula", "o_kaya_ay", "sa_pagpapatuloy", "pangkalahatan", "pass", "def", "kung", "itaas", "burahin", "ibilang", "ibalik", "o_kung_kaya", "sa", "tama", "kung_wala", "is", "subok"
		]
		__result__ = ""
		__data__ = __code__.split("\n")
		for __d__ in __data__:
			__data2__ = __d__.split(" ")
			for __d2__ in __data2__:
				__key__ = __d2__.replace(":", "")
				# if __d2__.startswith("(\""):
				# 	__result__+= "(\""
				# if __d2__.startswith("("):
				# 	__result__+= "("
				if __key__.replace(":", "").replace("(", "").replace(")", "") in tagalog:
					for i in range(len(tagalog)):
						if tagalog[i] == __key__.replace(":", "").replace("(", "").replace(")", ""):
							__result__ += reserved_keywords[i]
				else:
					__result__ += __key__
				
				# if not __d2__.endswith("\")") and __d2__.endswith(")"):
				# 	__result__+= ")"
				# if  __d2__.endswith("\")"):
				# 	__result__+= "\")"
				if __d2__.endswith("):") and not __d2__.endswith("\"):"):
					__result__+= ")"
				if __d2__.endswith(":"):
					__result__ += ":"
				__result__ += " "
			__result__ += "\n"
		return __result__
	
	def getResult(self):
		with open(self.file, "r") as f1:
			exec(self.__magsalin__(f1.read()))
	
	def checkCode(self):
		with open(self.file, "r") as f1:
			print(f1.read())