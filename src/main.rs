
mod cell;
mod refcell;
fn main(){

    let x = cell::Cell::new(2);

    let y = &x;
    let z = x.get();

    y.set(45);
    println!("{:?}" , x.get())
}