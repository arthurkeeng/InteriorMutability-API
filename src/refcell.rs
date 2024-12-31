use std::{cell::UnsafeCell, ops::{Deref, DerefMut}};
use crate::cell::{Cell};
#[derive(Clone, Copy)]
enum RefState{
    Unshared , Shared(u64) , Exclusive
}
pub struct RefCell<T>{
    
    
    value : UnsafeCell<T>,
    state : Cell<RefState>
}

impl<T> RefCell<T>{
    fn new(value : T) ->Self{
        RefCell { value : UnsafeCell::new(value), state: Cell::new(RefState::Unshared)  }
    }

    fn borrow(&self) ->Option<Ref<'_ ,T>>{
        match self.state.get(){
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref{
                    refcell : self
                })
                
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref{
                    refcell : self
                })
                  
            }
            RefState::Exclusive => {
                None
            }
        }
    }
    fn borrow_mut(&self) ->Option<RefMut<'_ , T>>{
        if let  RefState::Unshared = self.state.get(){

            self.state.set(RefState::Exclusive);
            Some(RefMut{
                refcell : self
            })
        }
        else{
            None
        }
       

    }
}

pub struct Ref<'refcell , T>{
    refcell : &'refcell RefCell<T>
}

impl<'refcell , T> Drop for Ref<'refcell , T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) =>{
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) =>{
                self.refcell.state.set(RefState::Shared(n - 1));
            }
            RefState::Unshared | RefState::Exclusive =>{
                unreachable!();
            }
        }
    }
    
}

impl<T> Deref for Ref<'_, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
      unsafe {
            
            &*self.refcell.value.get()
        }
        
    }
}

pub struct RefMut<'refcell , T>{
    
    refcell : &'refcell RefCell<T>
}
impl<'refcell , T> Drop for RefMut<'refcell , T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) =>{
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) =>{
                self.refcell.state.set(RefState::Shared(n - 1));
            }
            RefState::Unshared | RefState::Exclusive =>{
                unreachable!();
            }
        }
    }
    
}

impl<T> Deref for RefMut<'_, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
      unsafe {
            
            &*self.refcell.value.get()
        }
        
    }
}

impl<T> DerefMut for RefMut<'_,T>{
    // type Target = T;
    fn deref_mut(&mut self) -> &mut Self::Target {
        
  
      unsafe {
            
            &mut *self.refcell.value.get()
        }
        
    }
}


fn main(){}