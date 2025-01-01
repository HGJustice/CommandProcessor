use crate::errors::{CommandErrors, OperationErrors};

pub trait Commands<T>{
    fn execute(&self, data: &mut T) -> Result<(), CommandErrors>;
    fn undo(&self, data: &mut T) -> Result<(), CommandErrors>;
    fn redo(&self, data: &mut T) -> Result<(), CommandErrors>;
}

pub struct Operation<T> {
    pub data: T,
}

impl<T> Operation<T> {
    pub fn new(data: T) -> Operation<T>{
        Operation { data }
    }
}

impl Operation<u32> {
    pub fn increment(&mut self, amount: u32) -> Result<(), OperationErrors> {
        if amount == 0 {
            return Err(OperationErrors::CannotIncreaseByZero);
        }

        let result = self.data.checked_add(amount);
        if result.is_none(){
            return Err(OperationErrors::IntegerOverflowError);
        }
        self.data = result.expect("shoudl incremenet data by amount");
        Ok(())
    }

    pub fn decrement(&mut self, amount: u32) -> Result<(), OperationErrors>{
        if amount == 0 {
            return Err(OperationErrors::CannotDecreaseByZero);
        }
        let result = self.data.checked_sub(amount);
        if result.is_none(){
            return Err(OperationErrors::IntergerUnderflowError);
        }
        self.data = result.expect("should decrement data by amount");
        Ok(())
    }

}

impl Operation<String> {

}