// use std::collections::HashSet;
use rand::Rng;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
struct Value<'a> {
    data: f64, 
    gradient: f64, 
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

    fn addition(self: &'a mut Value<'a>, other: &'a mut Value<'a>) -> Value {
        // print_type_of(&self);
        self.gradient += 1.0;
        other.gradient += 1.0;
        Value {
            data: self.data + other.data, 
            gradient: 0.0,
            children: vec![self, other],
        }
    }

    fn power(self: &'a mut Value<'a>, other: i32) -> Value {
        // print_type_of(&self);
        self.gradient += f64::from(other) * f64::powi(self.data, other - 1);
        Value {
            data: f64::powi(self.data, other), 
            gradient: 0.0,
            children: vec![self],
        }
    }

    // just going to do chain rule
    fn backward(&mut self) {
        for node in &mut self.children {
            node.gradient = node.gradient * self.gradient;
            node.backward();
        }
    }
}

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
//             weights: Vec::from_iter(0..n).iter().map(|_| { rand::thread_rng().gen_range(-1.0..=1.0)}).collect::<Vec<f64>>(),
//             bias: 0.0,
//             parameters: Vec::new(),
//             nonlin: nonlin, 
//         }
//     }   
    
//     fn call(&self, n: Vec<f64>) -> Value {
//         let mut acc = self.bias;

//         for (x, y) in (self.weights).iter().zip(n.iter()) {
//             // we have to change this to be values
//             acc += x * y;    
//         }
//         acc
//         // write code for nonlin 
//     }

// }



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

    // println!("{:#?}", Neuron::new(5))
    println!("Now {:#?} will print!", &new_node);
    // new_node.children.get(0).unwrap().data = 0.2; 
}
