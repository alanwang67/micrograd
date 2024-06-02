use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
// whats the difference between use::borrow:borrow_mut and regular
#[derive(Debug)]
struct ValueData {
    data: f64, 
    gradient: f64, 
    children: Vec<Value>,
}

#[derive(Debug)]
struct Value(Rc<RefCell<ValueData>>);

impl Value {
    fn new(data: f64, gradient: f64, children: Vec<Value>) -> Self {
        Value(Rc::new(RefCell::new(ValueData {
            data, 
            gradient,
            children})))
       
    }

    // why do we need a mutable reference here, because we are mutating it's gradient
    fn multiplication(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; // when is data cloned
        let other_data = other.0.borrow().data;
        self.0.borrow_mut().gradient += other_data;
        other.0.borrow_mut().gradient += self_data;

        Value(Rc::new(RefCell::new(ValueData { data: self_data * other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))]})));
    }

    // fn add(self: &'a mut Value<'a>, other: &'a mut Value<'a>) -> Value {
    //     self.gradient += 1.0;
    //     other.gradient += 1.0;
    //     Value {
    //         data: self.data + other.data, 
    //         gradient: 0.0,
    //         children: vec![self, other],
    //     }
    // }

    // fn power(self: &'a mut Value<'a>, other: i32) -> Value {
    //     self.gradient += f64::from(other) * f64::powi(self.data, other - 1);
    //     Value {
    //         data: f64::powi(self.data, other), 
    //         gradient: 0.0,
    //         children: vec![self],
    //     }
    // }

    // just going to do chain rule
    // fn backward(&mut self) {
    //     for node in &mut self.children {
    //         node.gradient = node.gradient * self.gradient;
    //         node.backward();
    //     }
    // }
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<Value>, 
    bias: Value, 
    parameters: Vec<Value>, 
    nonlin: bool,
}

// impl<'a> Neuron<'a> {
//     fn new(n: u64, nonlin: bool) -> Self {
//         Neuron{
//             weights: Vec::from_iter(0..n).iter().map(|_| { 
//                 let data = rand::thread_rng().gen_range(-1.0..=1.0);
//                 Value::new(data, 0.0, vec![])

//             }).collect(),
//             bias: Value::new(0.0, 0.0, vec![]),
//             parameters: Vec::new(),
//             nonlin: nonlin, 
//         }
//     }   
    
//     // its implied n is mutable since we are giving ownership to the vector of values
//     fn call(&mut self, n: Vec<Value<'a>>) -> Value {
//         // we want to make a new copy of bias here? 
//         // how does borrow checking work 
//         let mut acc = Value::new(self.bias.data, 0.0, vec![]); // do we want to implement a trait here? 

//         let mut index = 0; 
//         for &mut x in self.weights { 
//             let dot_product = x.multiplication(&n[index]);
//             index += 1;
//         }

//         acc
//     }
// }


fn main() {
    // for now this will live on the stack 
    let v1 = Value::new(0.3, 0.0, Vec::new());
    let v2 = Value::new(0.5, 0.0, Vec::new());

    // let mut new_node = (&mut v).multiplication(&mut v1);
    // new_node.gradient = 1.0;
    // ((&mut new_node).backward());

    // println!("{:#?}", v1.multiplication(&v2))
    // println!("Now {:#?} will print!", &new_node);
    // new_node.children.get(0).unwrap().data = 0.2; 
}
