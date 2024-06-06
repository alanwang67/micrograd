mod engine;

mod nn;

use crate::engine::Value;
use crate::nn::Neuron;

fn main() {

    let neuron = Neuron::new(3, true);
    println!("{:#?}", neuron);

    for e in &neuron.weights {
        println!("{}", e);
    }
    
    let inputs = vec![Value::new(2.0), Value::new(3.0), Value::new(8.0)];
    println!("{:#?}", neuron.call(&inputs));
}

// #![ allow(warnings)]
// use std::cell::RefCell;
// use std::rc::Rc;
// use std::hash::{Hash, Hasher};
// use rand::Rng;
// use std::collections::HashSet;
// use std::collections::BTreeSet;
// use std::fmt;
// use by_address::ByAddress;


// // how do the hashing traits work?
// // how do we implement backward for value?
// // how can we write tests for what we have written so far?
// // how can we extract this into a module?
// // can we use this to train a basic loss

// // whats the difference between use::borrow:borrow_mut and regular
// #[derive(Debug)]
// struct ValueData {
//     data: f64, 
//     gradient: f64, 
//     children: Vec<Value>,
// }

// #[derive(Debug)]
// struct Value(Rc<RefCell<ValueData>>);

// impl Value {
//     fn new(data: f64) -> Self {
//         Value(Rc::new(RefCell::new(ValueData {
//             data, 
//             gradient: 0.0,
//             children: vec![]})))
//     }

//     fn multiply(&self, other: &Value) -> Self {
//         let self_data = self.0.borrow().data; 
//         let other_data = other.0.borrow().data;
//         self.0.borrow_mut().gradient += other_data;
//         other.0.borrow_mut().gradient += self_data;

//         Value(Rc::new(RefCell::new(ValueData { data: self_data * other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))]})))
//     }

//     fn add(&self, other: &Value) -> Self {
//         let self_data = self.0.borrow().data; 
//         let other_data = other.0.borrow().data;
//         self.0.borrow_mut().gradient += 1.0;
//         other.0.borrow_mut().gradient += 1.0;

//         Value(Rc::new(RefCell::new(ValueData { data: self_data + other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))]})))
//     }

//     fn power(&self, other: i32) -> Self {
//         let self_data = self.0.borrow().data; 
//         self.0.borrow_mut().gradient += f64::from(other) * f64::powi(self_data, other - 1);

//         Value(Rc::new(RefCell::new(ValueData { data: f64::powi(self_data, other), gradient: 0.0, children: vec![Value(Rc::clone(&self.0))]})))
//     }

//     fn relu(&self) -> Self {
//         let self_data = self.0.borrow().data; 
//         let data = if self_data < 0.0 {0.0} else {self_data};
//         let self_grad = if self_data > 0.0 {1.0} else {0.0};
//         self.0.borrow_mut().gradient += self_grad;
//         Value(Rc::new(RefCell::new(ValueData { data: data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0))]})))
//     }

//     fn backward(&self) {
//         let mut topo = vec![]; 
//         let mut visited = HashSet::new();
//         let mut stack: Vec<Value> = vec![Value(Rc::clone(&self.0))];

//         while stack.len() != 0 { 
//             let element: Value = match stack.pop() {
//                 None => panic!("We should not be in this case."), 
//                 Some(i) => i,
//             }; 
            
//             let r = Rc::clone(&element.0); 
            
//             // when we insert the address into the reference the value doesn't live long enough? 
//             // but that's not true since there are multiple Rc references
//             if !(visited).contains(&ByAddress(Rc::clone(&element.0))) { 
//                 visited.insert(ByAddress(r)); 
//                 let children: &Vec<Value> = &self.0.borrow().children; 
//                 for child in children { 
//                     stack.push(Value(Rc::clone(&child.0)));
//                 }
//                 topo.push(Rc::clone(&element.0));
//             }
//         }

//         println!("{:#?}", topo); 

        
//         //I was wondering what "for x in &v" does when v is a vector. I understand that it returns a pointer for every element 

//         // let self_gradient = self.0.borrow().gradient; 
//         // for node in &self.0.borrow().children {
//         //     let node_gradient = node.0.borrow_mut().gradient;
//         //     node.0.borrow_mut().gradient *= self_gradient;
//         //     node.backward();
//         // }
//     }
// }

// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0.borrow().data)
//     }
// }

// // do we want to wrap this neuron within a rc<refcell> 
// #[derive(Debug)]
// struct Neuron {
//     weights: Vec<Value>, 
//     bias: Value, 
//     parameters: Vec<Value>, 
//     nonlin: bool,
// }

// impl Neuron {
//     fn new(n: u64, nonlin: bool) -> Self {
//         Neuron{
//             weights: Vec::from_iter(0..n).iter().map(|_| { 
//                 let data = rand::thread_rng().gen_range(-1.0..=1.0);
//                 Value::new(data)

//             }).collect(),
//             bias: Value::new(0.0),
//             parameters: Vec::new(),
//             nonlin: nonlin, 
//         }
//     }   
    
//     fn call(&self, n: &Vec<Value>) -> Value {
//         let mut acc = Value(Rc::clone(&self.bias.0));
        
//         for (x, y) in (self.weights).iter().zip(n.iter()) {
//             acc = acc.add(&(x.multiply(&y)))
//         }

//         if self.nonlin { 
//             acc.relu()
//         }
//         else { 
//             acc
//         }
//     }

//     fn params(&self) -> Vec<Value> {
//         let bias = Value(Rc::clone(&self.bias.0));
//         let mut v = vec![bias];
//         for w in &self.weights {
//             v.push(Value(Rc::clone(&w.0)));
//         }
//         v
//     }

// }

// #[derive(Debug)]
// struct Layer {
//     neurons: Vec<Neuron>
// }

// impl Layer {
//     fn new(nin: u64, nout: u64, nonlin: bool) -> Self {
//         Layer { 
//             neurons: Vec::from_iter(0..nout).iter().map(|_| Neuron::new(nin, nonlin)).collect(),
//         }
//     }

//     fn call(&self, x: &Vec<Value>) -> Vec<Value> {
//         self.neurons.iter().map(|neuron| {neuron.call(x)}).collect::<Vec<Value>>()
//     }

//     fn params(&self) -> Vec<Value> {
//         let mut v = vec![];
//         for n in &self.neurons { 
//             for p in &n.params() {
//                 v.push(Value(Rc::clone(&p.0)));
//             }
//         }
//         v
//     }
// }

// struct MLP {
//     layers: Vec<Layer>
// }

// impl MLP {
//     // I think we need to put mutable on the second parameter 
//     // because it specifies what we can do with our vector even though
//     // we have ownership of it (something we have ownership over doesn't mean it's mutable)
//     fn new(nin: u64, mut nout: Vec<u64>) -> Self { 
//         let mut sz = vec![nin];
//         sz.append(&mut nout);
//         let mut layers = vec![];
//         let mut i = 0;

//         while i < nout.len() {
//             layers.push(Layer::new(*&sz[i], *&sz[i+1], i != (nout.len()-1)));
//             i += 1; // ? 
//         }

//         MLP {
//             layers: layers
//         }
//     }

//     fn call(&self, mut x: Vec<Value>) -> Vec<Value> {
//         for layer in &self.layers {
//             x = layer.call(&x);
//         }
//         x
//     }

//     fn params(&self) -> Vec<Value> {
//         let mut v = vec![];
//         for l in &self.layers { 
//             for p in &l.params() {
//                 v.push(Value(Rc::clone(&p.0)));
//             }
//         }
//         v
//     }

// }

// fn main() {
//     // for now this will live on the stack 
//     let v = Value::new(2.0);
//     let v1 = Value::new(6.0);

//     let mut new_node = v.multiply(&v1);
//     new_node.0.borrow_mut().gradient = 2.0;
//     ((&mut new_node).backward());

//     println!("{:#?}", new_node);
//     // println!("{:#?}", Neuron::new(2, false).params())
//     // println!("{:#?}", Neuron::new(2, false).call(vec![v1,v2]))


//     // let new_node = v1.multiply(&v2);
//     // new_node.0.borrow_mut().gradient = 2.0;
//     // new_node.params();
    
//     // let x = Value::new(-4.0);
//     // let z = ((Value::new(2.0).multiply(&x)).add(&Value::new(2.0))).add(&x);
//     // z.0.borrow_mut().gradient = 1.0;
//     // z.backward();

//     // let mut x = vec![Value::new(2.0),Value::new(3.0),Value::new(-1.0)];
//     // let n = MLP::new(2, vec![4,4,1]);
//     // let mut x = vec![Value::new(2.0),Value::new(3.0),V];
//     // let mut n = MLP::new(2, vec![4,4,1]);

//     // println!("{:#?}", n.call(&x));


//     // println!("{:#?}", z)


//     // println!("Now {:#?} will print!", &new_node);
//     // new_node.children.get(0).unwrap().data = 0.2; 
// }
