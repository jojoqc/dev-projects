enum eStates{
    Halt,
    Error,
    Warning,
    Done,
    List,
    Next,
    Previous,
    Jump,
}
struct sMachine{
    state: eStates,
    uuid: u16,
}
impl sMachine {
    fn new()->Self{
        sMachine{
            state: eStates::Done,
            uuid : 1,
        }
    }
    fn status(&self)->Self{
        return *self
    }
}

fn main(){
    let mut machine: sMachine = sMachine::new();
    println!("{:?}",machine);
}
