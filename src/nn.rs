use rand::Rng;
use std::{cell::RefCell, rc::Rc, fmt};
use crate::engine::Value;

#[derive(Debug)]
pub struct Neuron {
    pub weights: Vec<Value>, 
    pub bias: Value, 
    pub parameters: Vec<Value>, 
    pub nonlin: bool,
}

impl Neuron {
    pub fn new(n: u64, nonlin: bool) -> Self {
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
    
    pub fn call(&self, n: &Vec<Value>) -> Value {
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

    pub fn params(&self) -> Vec<Value> {
        let bias = Value(Rc::clone(&self.bias.0));
        let mut v = vec![bias];
        for w in &self.weights {
            v.push(Value(Rc::clone(&w.0)));
        }
        v
    }
}

#[derive(Debug)]
pub struct Layer {
    pub neurons: Vec<Neuron>
}

impl Layer {
    pub fn new(nin: u64, nout: u64, nonlin: bool) -> Self {
        Layer { 
            neurons: Vec::from_iter(0..nout).iter().map(|_| Neuron::new(nin, nonlin)).collect(),
        }
    }

    pub fn call(&self, x: &Vec<Value>) -> Vec<Value> {
        self.neurons.iter().map(|neuron| {neuron.call(x)}).collect::<Vec<Value>>()
    }

    pub fn params(&self) -> Vec<Value> {
        let mut v = vec![];
        for n in &self.neurons { 
            for p in &n.params() {
                v.push(Value(Rc::clone(&p.0)));
            }
        }
        v
    }
}

pub struct MLP {
    pub layers: Vec<Layer>
}

impl MLP {
    pub fn new(nin: u64, mut nouts: Vec<u64>) -> Self { 
        let length = nouts.len(); 
        let mut sz = vec![nin];
        sz.append(&mut nouts);        
        let mut layers = vec![];
        let mut i = 0;

        while i < length {
            layers.push(Layer::new(*&sz[i], *&sz[i+1], i != (length-1)));
            i += 1;
        }

        MLP {
            layers: layers
        }
    }

    pub fn call(&self, mut x: Vec<Value>) -> Vec<Value> {
        for layer in &self.layers {
            x = layer.call(&x);
        }
        x
    }

    pub fn params(&self) -> Vec<Value> {
        let mut v = vec![];
        for l in &self.layers { 
            for p in &l.params() {
                v.push(Value(Rc::clone(&p.0)));
            }
        }
        v
    }

}
