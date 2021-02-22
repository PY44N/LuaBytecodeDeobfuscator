local InFile = io.open('Out1.lua', 'rb')

local Script = InFile:read'*a'

InFile:close()

loadstring(Script)()