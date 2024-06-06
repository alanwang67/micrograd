use std::{cell::RefCell, rc::Rc, fmt, collections::HashSet};
use by_address::ByAddress;

#[derive(Debug)]
pub struct ValueData {
    pub data: f64, 
    pub gradient: f64, 
    pub children: Vec<Value>,
}

#[derive(Debug)]
pub struct Value(pub Rc<RefCell<ValueData>>);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "data: {}, gradient: {}", self.get_value(), self.get_gradient())
    }
}

impl Value {
    pub fn new(data: f64) -> Self {
        Value(Rc::new(RefCell::new(ValueData {
            data, 
            gradient: 0.0,
            children: vec![]})))
    }

    pub fn get_value(&self) -> f64 {
        self.0.borrow().data
    }

    pub fn get_gradient(&self) -> f64 {
        self.0.borrow().gradient
    }

    pub fn multiply(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; 
        let other_data = other.0.borrow().data;
        self.0.borrow_mut().gradient += other_data;
        other.0.borrow_mut().gradient += self_data;

        Value(Rc::new(RefCell::new(ValueData { data: self_data * other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))]})))
    }

    pub fn add(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; 
        let other_data = other.0.borrow().data;
        self.0.borrow_mut().gradient += 1.0;
        other.0.borrow_mut().gradient += 1.0;

        Value(Rc::new(RefCell::new(ValueData { data: self_data + other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))]})))
    }

    pub fn power(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; 
        let other_data = other.0.borrow().data as i32; 
        self.0.borrow_mut().gradient += f64::from(other_data) * f64::powi(self_data, other_data - 1);

        Value(Rc::new(RefCell::new(ValueData { data: f64::powi(self_data, other_data), gradient: 0.0, children: vec![Value(Rc::clone(&self.0))]})))
    }

    pub fn subtract(&self, other: &Value) -> Self {
        other.0.borrow_mut().data *= -1.0;
        self.add(other)
    }

    pub fn relu(&self) -> Self {
        let self_data = self.0.borrow().data; 
        let data = if self_data < 0.0 {0.0} else {self_data};
        let self_grad = if self_data > 0.0 {1.0} else {0.0};
        self.0.borrow_mut().gradient += self_grad;
        Value(Rc::new(RefCell::new(ValueData { data: data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0))]})))
    }

    fn topological_sort(element: Value, topo: &mut Vec<Value>, visited: &mut HashSet<ByAddress<Rc<RefCell<ValueData>>>>) { 
        let r = ByAddress(Rc::clone(&element.0));

        if !(visited).contains(&r) { 
            visited.insert(r); 
            let children: &Vec<Value> = &element.0.borrow().children; 
            for child in children { 
                Self::topological_sort(Value(Rc::clone(&child.0)), topo, visited);
            }
            topo.push(Value(Rc::clone(&element.0)));
        }
    }

    pub fn backward(&self) {
        let mut topo = vec![]; 
        let mut visited = HashSet::new();
        Self::topological_sort(Value(Rc::clone(&self.0)), &mut topo, &mut visited);

        topo.reverse();
        self.0.borrow_mut().gradient = 1.0;
        
        for element in &topo {
            let element_gradient = element.0.borrow().gradient;   
            for node in &element.0.borrow().children {
                node.0.borrow_mut().gradient *= element_gradient;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_pass() {
        let x = Value::new(-4.0);
        let y = Value::new(2.0);
        let z = Value::new(2.0);
        let result = (y.multiply(&x)).add(&(z.add(&x)));

        assert!(result.get_value() == -10.0);
    }

    #[test]
    fn test_backward_pass() {
        let x = Value::new(-4.0);
        let y = Value::new(2.0);
        let z = Value::new(2.0);
        let result = (y.multiply(&x)).add(&(z.add(&x))); // y * x + z + x
        result.backward();

        assert!(result.get_gradient() == 1.0);
        assert!(x.get_gradient() == 3.0);
        assert!(y.get_gradient() == -4.0);
        assert!(z.get_gradient() == 1.0);
    }
}