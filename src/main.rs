mod engine;

mod nn;

use crate::engine::Value;
use crate::nn::{Neuron, Layer, MLP};

fn main() {
    let mut xs = vec![vec![Value::new(2.0), Value::new(3.0), Value::new(-1.0)], vec![Value::new(3.0), Value::new(-1.0), Value::new(0.5)], vec![Value::new(0.5), Value::new(1.0), Value::new(1.0)], vec![Value::new(1.0), Value::new(1.0), Value::new(-1.0)]];
    let mut ys = vec![Value::new(1.0), Value::new(-1.0), Value::new(-1.0), Value::new(1.0)];

    let n = MLP::new(3, vec![4,4,1]);
    let mut data = vec![];
    let mut loss = Value::new(0.0);

    for x in xs {
        data.push(n.call(x).pop().unwrap()); // assuming only 1 element in vector
    }
    
    println!("{:#?}", data.iter().map(|value| value.0.borrow().data).collect::<Vec<f64>>());
    
    for (x,y) in data.iter().zip(ys.iter()) {
        let squared_difference = (y.subtract(x)).power(&Value::new(2.0));
        loss.add(&squared_difference);
    }

    println!("{:#?}", loss.backward());

    let mut i = 0;
    while i < 1 { 
        for p in n.params() {
            let p_data = p.0.borrow().data;
            p.0.borrow_mut().data += -0.01 * p_data;
        }

        loss.backward()

        for p in n.params() {
            let p_data = p.0.borrow().data;
            p.0.borrow_mut().data += -0.01 * p_data;
        }
        i += 1;
    }

    let mut xs = vec![vec![Value::new(2.0), Value::new(3.0), Value::new(-1.0)], vec![Value::new(3.0), Value::new(-1.0), Value::new(0.5)], vec![Value::new(0.5), Value::new(1.0), Value::new(1.0)], vec![Value::new(1.0), Value::new(1.0), Value::new(-1.0)]];

    for x in xs {
        data.push(n.call(x).pop().unwrap()); // assuming only 1 element in vector
    }

    println!("{:#?}", data.iter().map(|value| value.0.borrow().data).collect::<Vec<f64>>());

    // let value = MLP::new(3, vec![4,4,1]).call(x);
    // println!("{:#?}", &value[0].0.borrow().data);

    // let layer = Layer::new(2,3,true);
    // println!("{:#?}", layer.call(&x).iter().map(|value| value.0.borrow().data).collect::<Vec<f64>>());


    
}
