use std::{cell::RefCell, rc::Rc, fmt, collections::HashSet};
use by_address::ByAddress;

#[derive(Debug)]

pub struct ValueData 
{
    pub data: f64, 
    pub gradient: f64, 
    pub children: Vec<Value>,
    pub backward: Option<fn(Value) -> ()>,
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
            children: vec![],
            backward: Option::None,
        })))
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

        let backward_fn = |out_value: Value| {
            let children = &out_value.0.borrow().children;
            let out_gradient = out_value.get_gradient();

            let left_data = (&children[0]).0.borrow().data;
            let right_data = (&children[1]).0.borrow().data;

            (&children[0]).0.borrow_mut().gradient += right_data * out_gradient;
            
            (&children[1]).0.borrow_mut().gradient += left_data * out_gradient;
        };

        Value(Rc::new(RefCell::new(ValueData { data: self_data * other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))], backward: Option::Some(backward_fn) })))
    }

    pub fn add(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; 
        let other_data = other.0.borrow().data;
        
        let backward_fn = |out_value: Value| {
            let children = &out_value.0.borrow().children;
            let out_gradient = out_value.get_gradient();

            (&children[0]).0.borrow_mut().gradient += out_gradient;
            
            (&children[1]).0.borrow_mut().gradient += out_gradient;
        };

        Value(Rc::new(RefCell::new(ValueData { data: self_data + other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))], backward: Option::Some(backward_fn) })))
    }

    pub fn subtract(&self, other: &Value) -> Self {
        other.0.borrow_mut().data *= -1.0;
        self.add(other)
    }

    pub fn power(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; 
        let other_data = other.0.borrow().data as i32; 
        self.0.borrow_mut().gradient += f64::from(other_data) * f64::powi(self_data, other_data - 1);


        let backward_fn = |out_value: Value| {
            let children = &out_value.0.borrow().children;
            let left_data = (&children[0]).0.borrow().data;
            let right_data = (&children[1]).0.borrow().data as i32;
            let out_gradient = out_value.get_gradient();

            (&children[0]).0.borrow_mut().gradient += f64::from(right_data) * f64::powi(left_data, right_data - 1) * out_gradient;
        };

        Value(Rc::new(RefCell::new(ValueData { data: f64::powi(self_data, other_data), gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))], backward: Option::Some(backward_fn)})))
    }


    pub fn relu(&self) -> Self {
        let self_data = self.0.borrow().data; 
        let data = if self_data < 0.0 {0.0} else {self_data};

        let backward_fn = |out_value: Value| {
            let children = &out_value.0.borrow().children;
            let out_gradient: f64 = out_value.get_gradient();

            let child_data = (&children[0]).0.borrow().data;

            if child_data > 0.0 {
                (&children[0]).0.borrow_mut().gradient += out_gradient;
            } else {
                (&children[0]).0.borrow_mut().gradient += 0.0;
            }
        };

        Value(Rc::new(RefCell::new(ValueData { data: data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0))], backward: Option::Some(backward_fn) })))
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

    pub fn propogate_backward(&self) {
        let mut topo = vec![]; 
        let mut visited = HashSet::new();
        Self::topological_sort(Value(Rc::clone(&self.0)), &mut topo, &mut visited);

        topo.reverse();
        self.0.borrow_mut().gradient = 1.0;
        
        for element in &topo {
            match element.0.borrow().backward {
                None => (),
                Some(func) => func(Value(Rc::clone(&element.0)))
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

        let x = Value::new(8.0);
        let y = Value::new(2.0);
        let result = x.subtract(&y); 

        assert!(result.get_value() == 6.0);
    }

    #[test]
    fn test_backward_pass() {
        let x = Value::new(-4.0);
        let y = Value::new(2.0);
        let z = Value::new(2.0);
        let result = (y.multiply(&x)).add(&(z.add(&x))); // y * x + z + x

        result.propogate_backward();

        println!("{:#?}", result);
        assert!(result.get_gradient() == 1.0);
        assert!(x.get_gradient() == 3.0);
        assert!(y.get_gradient() == -4.0);
        assert!(z.get_gradient() == 1.0);
    }
}