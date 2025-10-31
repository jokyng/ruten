use crate::error::RutenError;
use crate::parser::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::modules;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    None,
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Environment,
    },
    NativeFunction(fn(&[Value]) -> Result<Value, RutenError>),
    Module(HashMap<String, Value>),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::None => "None".to_string(),
            Value::List(items) => {
                let strs: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Dict(map) => {
                let pairs: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            Value::Function { .. } => "<function>".to_string(),
            Value::NativeFunction(_) => "<native function>".to_string(),
            Value::Module(_) => "<module>".to_string(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::None => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn set(&mut self, name: &str, value: Value) -> Result<(), RutenError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        // if not found, define in current scope
        self.define(name.to_string(), value);
        Ok(())
    }
}

pub struct Interpreter {
    env: Environment,
    return_value: Option<Value>,
    break_flag: bool,
    continue_flag: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            env: Environment::new(),
            return_value: None,
            break_flag: false,
            continue_flag: false,
        };
        interpreter.init_builtins();
        interpreter
    }

    fn init_builtins(&mut self) {
        // print function
        self.env.define(
            "print".to_string(),
            Value::NativeFunction(|args| {
                let output = args
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("{}", output);
                Ok(Value::None)
            }),
        );

        // len function
        self.env.define(
            "len".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RutenError::RuntimeError(
                        "len() takes exactly 1 argument".to_string(),
                    ));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::Number(s.len() as f64)),
                    Value::List(l) => Ok(Value::Number(l.len() as f64)),
                    Value::Dict(d) => Ok(Value::Number(d.len() as f64)),
                    _ => Err(RutenError::TypeError(
                        "len() argument must be a string, list, or dict".to_string(),
                    )),
                }
            }),
        );

        // range function
        self.env.define(
            "range".to_string(),
            Value::NativeFunction(|args| {
                if args.is_empty() || args.len() > 3 {
                    return Err(RutenError::RuntimeError(
                        "range() takes 1 to 3 arguments".to_string(),
                    ));
                }
                
                let (start, end, step) = match args.len() {
                    1 => {
                        if let Value::Number(n) = args[0] {
                            (0.0, n, 1.0)
                        } else {
                            return Err(RutenError::TypeError("range() arguments must be numbers".to_string()));
                        }
                    }
                    2 => {
                        if let (Value::Number(s), Value::Number(e)) = (&args[0], &args[1]) {
                            (*s, *e, 1.0)
                        } else {
                            return Err(RutenError::TypeError("range() arguments must be numbers".to_string()));
                        }
                    }
                    3 => {
                        if let (Value::Number(s), Value::Number(e), Value::Number(st)) = (&args[0], &args[1], &args[2]) {
                            (*s, *e, *st)
                        } else {
                            return Err(RutenError::TypeError("range() arguments must be numbers".to_string()));
                        }
                    }
                    _ => unreachable!(),
                };

                let mut result = Vec::new();
                let mut current = start;
                while (step > 0.0 && current < end) || (step < 0.0 && current > end) {
                    result.push(Value::Number(current));
                    current += step;
                }
                Ok(Value::List(result))
            }),
        );

        // str function
        self.env.define(
            "str".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RutenError::RuntimeError(
                        "str() takes exactly 1 argument".to_string(),
                    ));
                }
                Ok(Value::String(args[0].to_string()))
            }),
        );

        // int function
        self.env.define(
            "int".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RutenError::RuntimeError(
                        "int() takes exactly 1 argument".to_string(),
                    ));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n.trunc())),
                    Value::String(s) => s
                        .parse::<f64>()
                        .map(|n| Value::Number(n.trunc()))
                        .map_err(|_| RutenError::RuntimeError("invalid literal for int()".to_string())),
                    _ => Err(RutenError::TypeError(
                        "int() argument must be a number or string".to_string(),
                    )),
                }
            }),
        );

        // float function
        self.env.define(
            "float".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err(RutenError::RuntimeError(
                        "float() takes exactly 1 argument".to_string(),
                    ));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::String(s) => s
                        .parse::<f64>()
                        .map(Value::Number)
                        .map_err(|_| RutenError::RuntimeError("invalid literal for float()".to_string())),
                    _ => Err(RutenError::TypeError(
                        "float() argument must be a number or string".to_string(),
                    )),
                }
            }),
        );
    }

    pub fn eval_program(&mut self, program: Program) -> Result<(), RutenError> {
        for stmt in program {
            self.eval_stmt(&stmt)?;
            if self.return_value.is_some() {
                break;
            }
        }
        Ok(())
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, RutenError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::None => Ok(Value::None),
            Expr::Identifier(name) => self
                .env
                .get(name)
                .ok_or_else(|| RutenError::NameError(format!("undefined variable: {}", name))),
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary_op(&left_val, op, &right_val)
            }
            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                self.eval_unary_op(op, &val)
            }
            Expr::Call { callee, args } => {
                let func = self.eval_expr(callee)?;
                let arg_vals: Result<Vec<_>, _> = args.iter().map(|a| self.eval_expr(a)).collect();
                let arg_vals = arg_vals?;
                self.call_function(func, arg_vals)
            }
            Expr::List(elements) => {
                let vals: Result<Vec<_>, _> = elements.iter().map(|e| self.eval_expr(e)).collect();
                Ok(Value::List(vals?))
            }
            Expr::Dict(pairs) => {
                let mut map = HashMap::new();
                for (key_expr, val_expr) in pairs {
                    let key = match self.eval_expr(key_expr)? {
                        Value::String(s) => s,
                        other => other.to_string(),
                    };
                    let val = self.eval_expr(val_expr)?;
                    map.insert(key, val);
                }
                Ok(Value::Dict(map))
            }
            Expr::Index { object, index } => {
                let obj = self.eval_expr(object)?;
                let idx = self.eval_expr(index)?;
                match (obj, idx) {
                    (Value::List(list), Value::Number(n)) => {
                        let index = n as i32;
                        let actual_index = if index < 0 {
                            (list.len() as i32 + index) as usize
                        } else {
                            index as usize
                        };
                        list.get(actual_index)
                            .cloned()
                            .ok_or_else(|| RutenError::RuntimeError("list index out of range".to_string()))
                    }
                    (Value::Dict(dict), Value::String(key)) => {
                        dict.get(&key)
                            .cloned()
                            .ok_or_else(|| RutenError::RuntimeError(format!("key not found: {}", key)))
                    }
                    (Value::String(s), Value::Number(n)) => {
                        let index = n as i32;
                        let actual_index = if index < 0 {
                            (s.len() as i32 + index) as usize
                        } else {
                            index as usize
                        };
                        s.chars()
                            .nth(actual_index)
                            .map(|c| Value::String(c.to_string()))
                            .ok_or_else(|| RutenError::RuntimeError("string index out of range".to_string()))
                    }
                    _ => Err(RutenError::TypeError("invalid index operation".to_string())),
                }
            }
            Expr::Member { object, member } => {
                let obj = self.eval_expr(object)?;
                match obj {
                    Value::Module(map) => map
                        .get(member)
                        .cloned()
                        .ok_or_else(|| RutenError::NameError(format!("module has no member: {}", member))),
                    _ => Err(RutenError::TypeError("member access on non-module".to_string())),
                }
            }
        }
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), RutenError> {
        if self.return_value.is_some() || self.break_flag || self.continue_flag {
            return Ok(());
        }

        match stmt {
            Stmt::Import(module) => {
                let module_value = modules::load_module(module)?;
                self.env.define(module.clone(), module_value);
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.set(name, val)?;
            }
            Stmt::FunctionDef { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.env.clone(),
                };
                self.env.define(name.clone(), func);
            }
            Stmt::Return(expr) => {
                self.return_value = Some(if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::None
                });
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval_expr(condition)?;
                if cond_val.is_truthy() {
                    for stmt in then_branch {
                        self.eval_stmt(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.eval_stmt(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                }
            }
            Stmt::While { condition, body } => {
                while self.eval_expr(condition)?.is_truthy() {
                    for stmt in body {
                        self.eval_stmt(stmt)?;
                        if self.return_value.is_some() {
                            return Ok(());
                        }
                        if self.break_flag {
                            self.break_flag = false;
                            return Ok(());
                        }
                        if self.continue_flag {
                            self.continue_flag = false;
                            break;
                        }
                    }
                }
            }
            Stmt::For { var, iterable, body } => {
                let iter_val = self.eval_expr(iterable)?;
                let items = match iter_val {
                    Value::List(items) => items,
                    Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
                    _ => {
                        return Err(RutenError::TypeError(
                            "for loop requires an iterable".to_string(),
                        ))
                    }
                };

                for item in items {
                    self.env.set(var, item)?;
                    for stmt in body {
                        self.eval_stmt(stmt)?;
                        if self.return_value.is_some() {
                            return Ok(());
                        }
                        if self.break_flag {
                            self.break_flag = false;
                            return Ok(());
                        }
                        if self.continue_flag {
                            self.continue_flag = false;
                            break;
                        }
                    }
                }
            }
            Stmt::Break => {
                self.break_flag = true;
            }
            Stmt::Continue => {
                self.continue_flag = true;
            }
            Stmt::Expression(expr) => {
                self.eval_expr(expr)?;
            }
        }

        Ok(())
    }

    fn eval_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, RutenError> {
        match (left, op, right) {
            (Value::Number(l), BinaryOp::Add, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), BinaryOp::Sub, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), BinaryOp::Mul, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), BinaryOp::Div, Value::Number(r)) => {
                if *r == 0.0 {
                    Err(RutenError::RuntimeError("division by zero".to_string()))
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            (Value::Number(l), BinaryOp::Mod, Value::Number(r)) => Ok(Value::Number(l % r)),
            (Value::String(l), BinaryOp::Add, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Number(l), BinaryOp::Equal, Value::Number(r)) => Ok(Value::Bool(l == r)),
            (Value::String(l), BinaryOp::Equal, Value::String(r)) => Ok(Value::Bool(l == r)),
            (Value::Bool(l), BinaryOp::Equal, Value::Bool(r)) => Ok(Value::Bool(l == r)),
            (Value::Number(l), BinaryOp::NotEqual, Value::Number(r)) => Ok(Value::Bool(l != r)),
            (Value::String(l), BinaryOp::NotEqual, Value::String(r)) => Ok(Value::Bool(l != r)),
            (Value::Bool(l), BinaryOp::NotEqual, Value::Bool(r)) => Ok(Value::Bool(l != r)),
            (Value::Number(l), BinaryOp::Less, Value::Number(r)) => Ok(Value::Bool(l < r)),
            (Value::Number(l), BinaryOp::LessEqual, Value::Number(r)) => Ok(Value::Bool(l <= r)),
            (Value::Number(l), BinaryOp::Greater, Value::Number(r)) => Ok(Value::Bool(l > r)),
            (Value::Number(l), BinaryOp::GreaterEqual, Value::Number(r)) => Ok(Value::Bool(l >= r)),
            (l, BinaryOp::And, r) => Ok(Value::Bool(l.is_truthy() && r.is_truthy())),
            (l, BinaryOp::Or, r) => Ok(Value::Bool(l.is_truthy() || r.is_truthy())),
            _ => Err(RutenError::TypeError(format!(
                "unsupported operation: {} {:?} {}",
                left.to_string(),
                op,
                right.to_string()
            ))),
        }
    }

    fn eval_unary_op(&self, op: &UnaryOp, val: &Value) -> Result<Value, RutenError> {
        match (op, val) {
            (UnaryOp::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, v) => Ok(Value::Bool(!v.is_truthy())),
            _ => Err(RutenError::TypeError(format!(
                "unsupported unary operation: {:?} {}",
                op,
                val.to_string()
            ))),
        }
    }

    fn call_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, RutenError> {
        match func {
            Value::NativeFunction(f) => f(&args),
            Value::Function {
                params,
                body,
                closure,
            } => {
                if params.len() != args.len() {
                    return Err(RutenError::RuntimeError(format!(
                        "function expects {} arguments, got {}",
                        params.len(),
                        args.len()
                    )));
                }

                // save current environment
                let saved_env = self.env.clone();
                self.env = closure;
                self.env.push_scope();

                // bind parameters
                for (param, arg) in params.iter().zip(args.iter()) {
                    self.env.define(param.clone(), arg.clone());
                }

                // execute function body
                for stmt in &body {
                    self.eval_stmt(stmt)?;
                    if self.return_value.is_some() {
                        break;
                    }
                }

                let result = self.return_value.take().unwrap_or(Value::None);
                
                // restore environment
                self.env = saved_env;

                Ok(result)
            }
            _ => Err(RutenError::TypeError("not a callable object".to_string())),
        }
    }
}