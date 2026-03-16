[FunctionDefinitionStatement("fn add(a, b) { body=[VariableAssignmentStatement("sum = InfixExpression(Integer(50) Math(Plus) InfixExpression(VariableExpression(name = "a") Math(Product) VariableExpression(name = "b")))"),FunctionCall(VariableExpression(name = "print")(VariableExpression(name = "sum")))], return=ReturnStatement("return VariableExpression(name = "sum")") }"),Text("
"),Text("
"),



FunctionDefinitionStatement("fn describePerson(person) { body=[IfBlock(condition=InfixExpression(ObjectExpression(value: VariableExpression(name = "person").VariableExpression(name = "age")) Comp(GreaterQuals) Integer(18)), valid=[Text("
"),Text("label = "adult""),Text("
"),Text("
"),Text("label = "minor""),Text("
")], invalid=[]),FunctionCall(VariableExpression(name = "print")(VariableExpression(name = "label")))], return=ReturnStatement("return VariableExpression(name = "label")") }"),Text("
")]
