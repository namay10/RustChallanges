use std::io::{self, Read};
use rand::Rng;
use std::cmp::Ordering;

pub enum Operator{
    Add,
    Sub,
    Mul,
    Div 
}
pub struct Calculator{
    a:i32,
    b:i32
}

impl Calculator{
   pub fn operation(&self,operator:Operator )->i32{
        let ans = match operator{
            Operator::Add=>self.a+self.b,
            Operator::Sub=>self.a-self.b,
            Operator::Mul=>self.a*self.b,
            Operator::Div=>{
                if self.b == 0 {
                    println!("Error: Division by zero is not allowed.");
                    0
                } else {
                    self.a / self.b
                }
            }
        };
        ans
    }
}

fn main(){
    let  a: i32=io::stdin().read(a).expect("enter an integer");
    let mut b: i32=io::stdin().read(& mut b).expect("enter an integer");
    let mut calculator= Calculator{
        a,
        b
    };
    let mut operator=String::new();
    io::stdin().read_line(& mut operator).expect("enter an operator");
    let result=calculator.operation(operator);
    }
