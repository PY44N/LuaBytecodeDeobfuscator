f = open("Input.lua", 'r')
script = f.read()
f.close()

script = script.replace("loadstring", "print")
script = script.replace("load", "print")
script = script.replace(")()", ")")

f = open("Out1.lua", "w")
f.write(script)
f.close()
