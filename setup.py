import argparse, os

parser = argparse.ArgumentParser(description="ᜀᜅ᜔ ᜐᜏ ᜀᜌ᜔ ᜁᜐᜅ᜔ ᜁᜐᜒᜈᜎᜒᜅ᜔ ᜏᜒᜃ ᜀᜆ᜔ ᜋᜈᜓᜐ᜔ᜃ᜔ᜇᜒᜆᜓ ᜈ ᜄᜎᜒᜅ᜔ ᜐ Python Programming Language᜶  ᜃᜌ ᜐᜏ ᜀᜌ᜔ ᜇᜑᜒᜎ᜔ ᜁᜆᜓ ᜈ ᜋᜒᜐ᜔ᜋᜓ ᜀᜅ᜔ ᜐᜒᜌᜅ᜔ ᜆᜄᜎᜓᜄ᜔ ᜈᜅ᜔ Python᜵  ᜀᜆ᜔ ᜂᜉᜅ᜔ ᜑᜒᜈ᜔ᜇᜒ ᜈ ᜇᜒᜈ᜔ ᜋᜑᜒᜇᜉᜈ᜔ ᜉ᜶")
parser.add_argument("name", help="--usage")

args = parser.parse_args().name

def replTo(txt):
	return txt.replace("ᜁᜆᜎ(", "print(").replace("ᜉᜑᜒᜅᜒ(", "input(").replace("ᜃᜓᜈᜅ᜔᜔᜔ ", "if ").replace("ᜆᜓᜆᜓᜂ", "True").replace("ᜂ ᜑᜒᜈ᜔ᜇᜒ ᜃᜌ ᜀᜌ᜔ ", "elif  ").replace("ᜃᜓᜅ᜔ ᜑᜒᜈ᜔ᜇᜒ ᜈᜋᜈ᜔", "else")

def replFrom(txt):
	return txt.replace("print(", "ᜁᜆᜎ(").replace("input(", "ᜉᜑᜒᜅᜒ(").replace("if ", "ᜃᜓᜈᜅ᜔᜔᜔ ").replace("True", "ᜆᜓᜆᜓᜂ").replace("elif  ", "ᜂ ᜑᜒᜈ᜔ᜇᜒ ᜃᜌ ᜀᜌ᜔ ").replace("else", "ᜃᜓᜅ᜔ ᜑᜒᜈ᜔ᜇᜒ ᜈᜋᜈ᜔")

if not args.endswith(".baybay"):
	args += ".baybay"
if os.path.exists(f"{args}"):
	with open(f"{args}", "r", encoding="UTF-8", errors="ignore") as code:
		c = code.read()
		c = c.replace("ᜁᜆᜎ(", "print(").replace("ᜉᜑᜒᜅᜒ(", "input(").replace("ᜃᜓᜈᜅ᜔᜔᜔ ", "if ").replace("ᜆᜓᜆᜓᜂ", "True").replace("ᜂ ᜑᜒᜈ᜔ᜇᜒ ᜃᜌ ᜀᜌ᜔ ", "elif  ").replace("ᜃᜓᜅ᜔ ᜑᜒᜈ᜔ᜇᜒ ᜈᜋᜈ᜔", "else")
		try:
			exec(c)
		except IndentationError as e:
			print(f"ᜑᜒᜈ᜔ᜇᜒ ᜐᜒᜌ ᜉᜈ᜔ᜆᜌ᜔ ᜃᜉᜆᜒᜇ᜔ \"{replFrom(e.text)}\" ᜐ line {e.lineno}")
		except SyntaxError as e:
			print(f"ᜋᜎᜒ ᜀᜅ᜔᜔ \"{replFrom(e.text)}\" ᜁᜆᜓ ᜀᜌ᜔ ᜑᜒᜈ᜔ᜇᜒ ᜋᜃᜒᜃᜒᜆ᜵  ᜋᜄᜒᜅ᜔ ᜋᜐᜒᜐᜒᜎᜌᜈ᜔ ᜐ ᜀᜃᜒᜅ᜔᜔ program᜶  ᜋᜃᜒᜃᜒᜆ ᜁᜆᜓ ᜐ line: {e.lineno}")
		except NameError as e:
			print(f"ᜋᜎᜒ ᜂᜎᜒᜆ᜔᜵  ᜑᜒᜈ᜔ᜇᜒ ᜃᜓ ᜀᜎᜋ᜔ ᜃᜓᜅ᜔ ᜀᜈᜓᜅ᜔᜔ \"{replFrom(e.name)}\" ᜁᜆᜓ")
else:
	print("ᜀᜅ᜔ ᜊᜄᜌ᜔ ᜈ ᜃᜁᜎᜅᜈ᜔ ᜈᜋᜒᜈ᜔ ᜀᜌ᜔ ᜑᜒᜈ᜔ᜇᜒ ᜋᜃᜒᜆ ᜐ ᜑᜈᜌ᜔ ᜈᜅ᜔ ᜋᜅ ᜇᜓᜃᜓᜋᜒᜈ᜔ᜆᜓ᜶  ᜋᜇᜉᜆ᜔ ᜈ ᜆᜒᜅ᜔ᜈᜈ᜔ ᜃᜓᜅ᜔ ᜆᜓᜋᜓᜆᜓᜄ᜔ᜋ ᜀᜅ᜔ ᜊᜏᜆ᜔ ᜆᜒᜆᜒᜃ᜔ ᜈᜅ᜔ ᜋᜅ ᜁᜆᜓ᜶")