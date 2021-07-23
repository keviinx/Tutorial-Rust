#[derive(Debug)]

struct Students {
     name:String,
     english:i32,
     math:i32,
     physics:i32,
}

impl Students{
    fn build(name:String, english:i32, math:i32, physics:i32)->Students
    {
        Students{name,english,math,physics}
    }

   fn best_mark(&self)->i32
    {
        if self.english < self.math
        {
            if self.english < self.physics
            {
                self.english
            }
            else
            {
                self.physics
            }
        }
        else 
        {
            if self.math < self.physics 
            {
                self.math
            }
            else
            {
                self.physics
            }
        }
    }

}

fn main() {
    let rob = Students{name:String::from("Rob"), english:2,math:3,physics:4};
    let flo = Students::build(String::from("FLO"),2,1,2);

    println!("The students are:  {:?} \n {:?}",rob,flo);

    println!("Best mark of {:?}:{}",flo.name,Students::best_mark(&flo));
    println!("Best mark of {:?}:{}",rob.name,Students::best_mark(&rob));
}
