-- usage: krait -e time.lua <number of iterations> <program> <arguments>

-- get the command line arguments
local args = krait.args
local iterations = tonumber(args[1])
local program = args[2]
local program_args = {}

-- get the program arguments
for i = 3, #args do
	program_args[i - 2] = args[i]
end

-- run the program
local start = krait.sys.clock()
for i = 1, iterations do
	krait.program.exec(program, program_args)
end
local finish = krait.sys.clock()

-- print the time
print("time: " .. (finish - start) .. " seconds")