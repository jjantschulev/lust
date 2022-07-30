use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression {
    Literal(bool),
    Variable(String),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Imp(Box<Expression>, Box<Expression>),
}

type Assignment = HashMap<String, bool>;

#[derive(Debug, PartialEq, Eq)]
pub enum EvaluationError {
    VariableNotAssigned(String),
}

impl Expression {
    pub fn eval(&self, assignment: &Assignment) -> Result<bool, EvaluationError> {
        match self {
            Expression::Literal(x) => Ok(*x),
            Expression::Variable(var) => match assignment.get(var) {
                Some(val) => Ok(*val),
                None => Err(EvaluationError::VariableNotAssigned(var.clone())),
            },
            Expression::And(a, b) => Ok(a.eval(assignment)? && b.eval(assignment)?),
            Expression::Or(a, b) => Ok(a.eval(assignment)? || b.eval(assignment)?),
            Expression::Not(e) => Ok(!e.eval(assignment)?),
            Expression::Imp(a, b) => Ok(!(a.eval(assignment)?) && !b.eval(assignment)?),
        }
    }
}

#[test]
fn evaluate_expression() {
    use Expression::*;
    let exp = And(Box::new(Literal(false)), Box::new(Literal(true)));
    assert_eq!(exp.eval(&HashMap::new()), Ok(false));
}
