from curses.ascii import isdigit
from numpy import outer
import regex

f = open("Input.lua", 'r')
script = f.read()
f.close()


pattern = regex.compile(r"""(?<=(loadstring|load)\(("|'|\[\[)).+(?=("|'|\]\])\)\(\))""")

bytecode = pattern.search(script)

output = ""

for i in bytecode.group(0).split("\\"):
    try: #Why can't python just let me check if something can be cast to an int?
        output += chr(int(i))
    except:
        continue
        

f = open("Out.lua", "w")
f.write(output)
f.close()
