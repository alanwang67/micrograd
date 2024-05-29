// use std::collections::HashSet;
use rand::Rng;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
struct Value<'a> {
    data: f64, 
    gradient: f64, 
    // similar to linked list structure let's get this to work 
    children: Vec<&'a mut Value<'a>>,
}

impl<'a> Value<'a> {
    //fn backward(&self) -> u32 {
        // recursively go through the children attached
    //}
    // fn create(data, gradient, children) -> Self {
    //     Value {
    //         data: data, 
    //         gradient: gradient,
    //         children: vec![],
    //     }
    // }

    fn multiplication(self: &'a mut Value<'a>, other: &'a mut Value<'a>) -> Value {
        // print_type_of(&self);
        self.gradient += other.data;
        other.gradient += self.data;
        Value {
            data: self.data * other.data, 
            gradient: 0.0,
            children: vec![self, other],
        }
    }

    // fn addition(self: &mut Value, other: &mut Value) -> Value {
    //     // print_type_of(&self);
    //     self.gradient += 1.0;
    //     other.gradient += 1.0;
    //     Value {
    //         data: self.data + other.data, 
    //         gradient: 1.0,
    //         children: vec![self, other],
    //     }
    // }

    // fn backward(v : Value) -> Value {
    //     // let seen = HashSet::new();
    //     for mut node in v.children {
    //         // if !(seen.contains(&node)) {
    //             // seen.insert(node);
    //         node.gradient = v.gradient * node.gradient;
    //         Self::backward(node);
    //         // }
    //     }
    //     v
    // }


    // just going to do chain rule
    fn backward(&mut self) {
        for node in &mut self.children {
            node.gradient = node.gradient * self.gradient;
            node.backward();
        }

    }
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<f64>, 
    bias: f64,
    parameters: Vec<f64>
}

impl Neuron {
    fn new(n: u64) -> Self {
        Neuron{
            weights: Vec::from_iter(0..n).iter().map(|_| { rand::thread_rng().gen_range(-1.0..=1.0)}).collect::<Vec<f64>>(),
            bias: 0.0,
            parameters: Vec::new(),
        }
    }   
    
    fn call(self, n : Vec<f64>) {
        
    }
}



fn main() {
    let mut v = Box::new(Value {
        data: 0.3,
        gradient: 0.0,
        children: Vec::new(),
    });
    let mut v1 = Box::new(Value {
        data: 0.9,
        gradient: 0.0,
        children: Vec::new(),
    });

    let mut new_node = (&mut v).multiplication(&mut v1);
    new_node.gradient = 1.0;
    ((&mut new_node).backward());

    println!("{:#?}", Neuron::new(5));
    println!("Now {:#?} will print!", &new_node);
    // new_node.children.get(0).unwrap().data = 0.2; 
}
