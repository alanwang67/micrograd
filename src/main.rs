use std::cell::RefCell;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use rand::Rng;
use std::collections::HashSet;

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
    fn new(data: f64) -> Self {
        Value(Rc::new(RefCell::new(ValueData {
            data, 
            gradient: 0.0,
            children: vec![]})))
       
    }

    // why do we need a mutable reference here, because we are mutating it's gradient
    fn multiply(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; // when is data cloned
        let other_data = other.0.borrow().data;
        self.0.borrow_mut().gradient += other_data;
        other.0.borrow_mut().gradient += self_data;

        Value(Rc::new(RefCell::new(ValueData { data: self_data * other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))]})))
    }

    fn add(&self, other: &Value) -> Self {
        let self_data = self.0.borrow().data; 
        let other_data = other.0.borrow().data;
        self.0.borrow_mut().gradient += 1.0;
        other.0.borrow_mut().gradient += 1.0;

        Value(Rc::new(RefCell::new(ValueData { data: self_data + other_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0)), Value(Rc::clone(&other.0))]})))
    }

    fn power(&self, other: i32) -> Self {
        let self_data = self.0.borrow().data; 
        self.0.borrow_mut().gradient += f64::from(other) * f64::powi(self_data, other - 1);

        Value(Rc::new(RefCell::new(ValueData { data: f64::powi(self_data, other), gradient: 0.0, children: vec![Value(Rc::clone(&self.0))]})))
    }

    fn relu(&self) -> Self {
        let self_data = self.0.borrow().data; 
        let data = if self_data < 0.0 {0.0} else {self_data};
        let self_grad = if self_data > 0.0 {1.0} else {0.0};
        self.0.borrow_mut().gradient += self_grad;
        Value(Rc::new(RefCell::new(ValueData { data: self_data, gradient: 0.0, children: vec![Value(Rc::clone(&self.0))]})))
    }

    // this will recurse infinitley if there are any overlaps 
    fn backward(&self) {
        let self_gradient = self.0.borrow().gradient; 
        for node in &self.0.borrow().children {
            let node_gradient = node.0.borrow_mut().gradient;
            node.0.borrow_mut().gradient *= self_gradient;
            node.backward();
        }
    }
}


// impl Hash for ValueData {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0.borrow().hash(state);
//     }
// }

// do we want to wrap this neuron within a rc<refcell> 
#[derive(Debug)]
struct Neuron {
    weights: Vec<Value>, 
    bias: Value, 
    parameters: Vec<Value>, 
    nonlin: bool,
}

impl Neuron {
    fn new(n: u64, nonlin: bool) -> Self {
        Neuron{
            weights: Vec::from_iter(0..n).iter().map(|_| { 
                let data = rand::thread_rng().gen_range(-1.0..=1.0);
                Value::new(data)

            }).collect(),
            bias: Value::new(0.0),
            parameters: Vec::new(),
            nonlin: nonlin, 
        }
    }   
    
    fn call(&self, n: &Vec<Value>) -> Value {
        let mut acc = Value(Rc::clone(&self.bias.0));
        
        for (x, y) in (self.weights).iter().zip(n.iter()) {
            acc = acc.add(&(x.multiply(&y)))
        }

        if self.nonlin { 
            acc.relu()
        }
        else { 
            acc
        }
    }

    fn params(&self) -> Vec<Value> {
        let bias = Value(Rc::clone(&self.bias.0));
        let mut v = vec![bias];
        for w in &self.weights {
            v.push(Value(Rc::clone(&w.0)));
        }
        v
    }

}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>
}

impl Layer {
    fn new(nin: u64, nout: u64, nonlin: bool) -> Self {
        Layer { 
            neurons: Vec::from_iter(0..nout).iter().map(|_| Neuron::new(nin, nonlin)).collect(),
        }
    }

    fn call(&self, x: &Vec<Value>) -> Vec<Value> {
        self.neurons.iter().map(|neuron| {neuron.call(x)}).collect::<Vec<Value>>()
    }

    fn params(&self) -> Vec<Value> {
        let mut v = vec![];
        for n in &self.neurons { 
            for p in &n.params() {
                v.push(Value(Rc::clone(&p.0)));
            }
        }
        v
    }
}

struct MLP {
    layers: Vec<Layer>
}

impl MLP {
    // I think we need to put mutable on the second parameter 
    // because it specifies what we can do with our vector even though
    // we have ownership of it (something we have ownership over doesn't mean it's mutable)
    fn new(&self, nin: u64, mut nout: Vec<u64>) -> Self { 
        let mut sz = vec![nin];
        sz.append(&mut nout);
        let mut layers = vec![];
        let mut i = 0;

        while i < nout.len() {
            layers.push(Layer::new(*&sz[i], *&sz[i+1], i != (nout.len()-1)));
        }

        MLP {
            layers: layers
        }
    }

    fn call(&self, mut x: Vec<Value>) -> Vec<Value> {
        for layer in &self.layers {
            x = layer.call(&x);
        }
        x
    }

    fn params(&self) -> Vec<Value> {
        let mut v = vec![];
        for l in &self.layers { 
            for p in &l.params() {
                v.push(Value(Rc::clone(&p.0)));
            }
        }
        v
    }

}

fn main() {
    // for now this will live on the stack 
    let v1 = Value::new(2.0);
    let v2 = Value::new(6.0);

    // let mut new_node = (&mut v).multiplication(&mut v1);
    // new_node.gradient = 1.0;
    // ((&mut new_node).backward());

    // println!("{:#?}", v1.multiplication(&v2))
    // println!("{:#?}", Neuron::new(2, false).params())
    // println!("{:#?}", Neuron::new(2, false).call(vec![v1,v2]))


    // let new_node = v1.multiply(&v2);
    // new_node.0.borrow_mut().gradient = 2.0;
    // new_node.params();
    
    let x = Value::new(-4.0);
    let z = ((Value::new(2.0).multiply(&x)).add(&Value::new(2.0))).add(&x);
    z.0.borrow_mut().gradient = 1.0;
    z.backward();



    println!("{:#?}", z)


    // println!("Now {:#?} will print!", &new_node);
    // new_node.children.get(0).unwrap().data = 0.2; 
}
