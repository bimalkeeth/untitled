
trait Quak{
    fn quack(&self);
}
struct Duck();
impl Quak for Duck{
    fn quack(&self) {
       println!("quack!!!!")
    }
}
struct RandomBird{
    is_a_parrot:bool
}

impl Quak for RandomBird{
    fn quack(&self) {
        if !self.is_a_parrot{
            println!("quack!!")
        }else {
            println!("squawk!!!")
        }
    }
}

fn main() {
    let duck1=Duck();
    let duck2=RandomBird{is_a_parrot:false};
    let parrot=RandomBird{is_a_parrot:true};
    let ducks:Vec<&dyn Quak>=vec![&duck1, &duck2, &parrot];
    for d in &ducks{
        d.quack();
    }
}

