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

impl Commands<u32> for Operation<u32> {

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
        self.data = result.expect("should incremenet data by amount");
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

impl Commands<String> for Operation<String>{

}

impl Operation<String> {
    pub fn append(&mut self, input: &str ) -> Result<(), OperationErrors>{
        if input.len() == 0 as usize {
            return Err(OperationErrors::InputStringIsEmpty);
        }
        self.data.push_str(input);
        Ok(())
    } 

    pub fn truncate(&mut self, amount: usize) -> Result<(), OperationErrors> {
        if amount == 0  {
            return Err(OperationErrors::CannotRemoveZeroCharacters);
        }
        if amount > self.data.len(){
            return Err(OperationErrors::AmountLargerThenString);
        }
        let result = self.data.len() - amount;
        self.data.truncate(result);
        Ok(())
    }
}