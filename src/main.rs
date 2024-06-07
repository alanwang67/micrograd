mod engine;

mod nn;

use crate::engine::Value;
use crate::nn::{MLP};

fn main() {
    let mut i = 0;
    let n = MLP::new(3, vec![4,4,1]);

    while i < 1000 {    
        let xs: Vec<Vec<Value>> = vec![vec![Value::new(2.0), Value::new(3.0), Value::new(-1.0)], vec![Value::new(3.0), Value::new(-1.0), Value::new(0.5)], vec![Value::new(0.5), Value::new(1.0), Value::new(1.0)], vec![Value::new(1.0), Value::new(1.0), Value::new(-1.0)]];
        let predict: Vec<Value> = vec![Value::new(1.0), Value::new(-1.0), Value::new(-1.0), Value::new(1.0)];

        let mut data: Vec<Value> = vec![];
        let mut loss = Value::new(0.0);

        for x in xs {
            data.push(n.call(x).pop().unwrap()); // assuming only 1 element in vector
        }

        println!("current predictions: {:#?}", data.iter().map(|value| value.0.borrow().data).collect::<Vec<f64>>());

        for (x,y) in data.iter().zip(predict.iter()) {
            let squared_difference = (y.subtract(x)).power(&Value::new(2.0));
            loss = loss.add(&squared_difference);
        }

        println!("current loss: {:#?}", loss.get_value());

        for p in n.params() {
            p.0.borrow_mut().gradient = 0.0;
        }

        loss.propogate_backward();

        for p in n.params() {
            let p_grad = p.get_gradient();
            p.0.borrow_mut().data += 0.01 * p_grad;
        }

        i += 1;
    }
}
