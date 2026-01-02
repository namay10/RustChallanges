// to do list 
use std::io;
struct Task{
    name:String,
    status:bool,
    desc:String,
}

impl Task{
    fn print(&self){
        format!("name:{}\nstatus:{}\ndescription:{}\n",self.name,self.status,self.desc);
    }
    fn update(&mut self){
        self.status=true;
    }
}
fn Input_getter(a:&str)->String{
    println!("Enter the {}",a);
    let mut input=String::new();
    io::stdin().read_line(& mut input).expect("enter a valid entry");
    input.trim().to_string()
}



fn main(){
    let mut tasks= Vec::<Task>::new();
    let mut name=Input_getter("name");
    let mut desc=Input_getter("description");
    let mut task=Task{
        name,
        desc,
        status:false
    };
    tasks.push(task);

    for t in tasks{
        t.print();
    }
}