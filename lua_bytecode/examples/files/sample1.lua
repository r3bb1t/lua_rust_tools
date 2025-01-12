-- Function to add two numbers
function add(a, b)
	print("hello1")
	print("hello")
	return a + b
end

-- Function to subtract two numbers
function subtract(a, b)
	return a - b
end

-- Function to multiply two numbers
function multiply(a, b)
	return a * b
end

-- Function to divide two numbers
function divide(a, b)
	if b == 0 then
		return "Error: Division by zero!"
	else
		return a / b
	end
end

-- Function to determine the type of day
function dayType(day)
	if day == 1 then
		return "Monday: Start of the work week."
	elseif day == 2 then
		return "Tuesday: Second day of the work week."
	elseif day == 3 then
		return "Wednesday: Midweek day."
	elseif day == 4 then
		return "Thursday: Almost the weekend."
	elseif day == 5 then
		return "Friday: Last work day of the week!"
	elseif day == 6 then
		return "Saturday: Weekend!"
	elseif day == 7 then
		return "Sunday: Weekend!"
	else
		return "Invalid day! Please enter a number between 1 and 7."
	end
end

-- Function to reverse a string
function reverseString(str)
	return str:reverse()
end

-- Example usage
print("Addition: " .. add(5, 3)) -- Output: Addition: 8
print("Subtraction: " .. subtract(10, 4)) -- Output: Subtraction: 6
print("Multiplication: " .. multiply(3, 7)) -- Output: Multiplication: 21
print("Division: " .. divide(8, 2)) -- Output: Division: 4
print("Division by zero: " .. divide(8, 0)) -- Output: Error: Division by zero!
print(dayType(3)) -- Output: Wednesday: Midweek day.
print(reverseString("Hello, World!")) -- Output: !dlroW ,olleH
