use std::ops::Deref;
use crate::cell::Cell;
struct RcInner<T>{
    value : T, 
    refcount : Cell<usize>
}

pub struct Rc<T>{
    inner : *const RcInner<T>
}

impl<T> Rc<T>{
    fn new(v : T)->Self{
        let inner = Box::new(RcInner{
            value : v , refcount : Cell::new(1)
        });

        Rc{
            inner : Box::into_raw(inner)
        }
    }
}


impl<T> Clone for Rc<T>{
    fn clone(&self) -> Self {
        let inner = unsafe {
            
            &*self.inner
        };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc{
            inner : self.inner
        }
    }
}

impl <T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe {&*self.inner}.value
    }
    
}
impl<T> Drop for Rc<T>{
    fn drop(&mut self) {
        let inner = unsafe{&*self.inner};
        let c = inner.refcount.get();

        if c == 1{
            drop(inner);
            let _ = unsafe {
                
                // Box::from_raw(*self.inner.)
            };
        }
        else{
            inner.refcount.set(c - 1);
        }
    }
}
fn main(){}