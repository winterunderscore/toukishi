--TODO: Looks ugly.
local exprs = {
	{"Binary", {{"Expr", "left"}, {"Token", "operator"}, {"Expr", "right"}}},
	{"Grouping", {{"Expr", "expression"}}},
	{"Literal", {{"Literal", "value"}}},
	{"Unary", {{"Token", "operator"}, {"Expr", "right"}}},
}

print("use super::lexer::{ Token, Literal };\n")
print("pub trait Visitor<T> {")

for _, expr in pairs(exprs) do
	local func_name = "visit_"..expr[1]:lower()
	local arguments = {}
	for i, argument in pairs(expr[2]) do
		arguments[i] = argument[2]..": &"..argument[1]
	end
	print("\tfn "..func_name.."(&mut self, "..table.concat(arguments, ", ").. ") -> T;")
end

print("}\n")

print("pub trait Acceptor<T> {")
print("\tfn accept(&self, visitor: &mut dyn Visitor<T>) -> T;")
print("}\n")

print("pub enum Expr {")

for _, expr in pairs(exprs) do
	local properties = {}
	for i, property in pairs(expr[2]) do
		properties[i] = property[2]..": "..(property[1] == "Expr" and "Box<Expr>" or property[1])
	end
	print("\t"..expr[1].." {")
	print("\t\t"..table.concat(properties, ",\n\t\t")..",")
	print("\t},")
end

print("}\n");

print("impl <T> Acceptor<T> for Expr {")
print("\tfn accept(&self, visitor: &mut dyn Visitor<T>) -> T {")
print("\t\tmatch self {")

for _, expr in pairs(exprs) do
	local arguments = {}
	for i, argument in pairs(expr[2]) do
		arguments[i] = argument[2]
	end
	arguments = table.concat(arguments, ", ")
	print(table.concat({
		"\t\t\tExpr::",
		expr[1],
		" { ",
		arguments,
		" } => visitor.visit_",
		(expr[1]):lower(),
		"(",
		arguments,
		"),"
	}, ""))
end

print("\t\t}")
print("\t}")
print("}")
