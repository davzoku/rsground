// Ownership: vec doesnt have copy trait.
// v originally has the ownership, of which it passes to v2.
// printing v will produce an error. 

fn main(){
    let v = vec![1,2,3];
    let v2 = v;
    
    //println!("v[0] is {}", v[0]);
    println!("v2[0] is {}", v2[0]);
}