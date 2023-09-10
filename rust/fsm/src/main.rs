#[allow(non_camel_case_types)]
//#[derive(Debug)]
//#[derive(Copy, Clone)]
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
#[allow(non_camel_case_types)]
//#[derive(Debug)]
//#[derive(Copy, Clone)]
struct sMachine{
    state: eStates,
    uuid: u16,
}

//#[allow(non_camel_case_types)]
//trait tStates{
//    fn new()->Self;
//    fn status(&self)->Self;
    //fn set(&self)->Self; 
//}

#[allow(non_camel_case_types)]
//impl tStates for sMachine{
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
