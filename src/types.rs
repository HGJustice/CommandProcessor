use std::process::Command;
use crate::errors::{CommandErrors, OperationErrors};

pub trait Commands<T>{
    fn execute(&mut self, operation: Operations) -> Result<(), CommandErrors>;
    fn undo(&mut self,) -> Result<(), CommandErrors>;
    fn redo(&mut self,) -> Result<(), CommandErrors>;
}

pub enum Operations {
    Increment(u32),
    Decrement(u32),
    Append(String),
    Truncate(usize),
}

impl Operations {
    fn increase(value: &mut u32, amount: u32) -> Result<u32, CommandErrors>{
        if amount == 0 {
            return Err(CommandErrors::CannotIncreaseByZero);
        }
        let result = value.checked_add(amount);
        if result.is_none(){
            return Err(CommandErrors::IntegerOverflowError);
        }
        Ok(result.expect("should increase the value by amount"))
    }

    fn decrease(value: &mut u32, amount: u32) -> Result<u32, CommandErrors>{
        if amount == 0 {
            return Err(CommandErrors::CannotDecreaseByZero);
        }
        let result = value.checked_sub(amount);
        if result.is_none(){
            return Err(CommandErrors::IntergerUnderflowError);
        }
        Ok(result.expect("should decrease the value by amount"))
    }

    fn append(value: &mut String, input: &str) -> Result<String, OperationErrors>{
        if input.is_empty() {
            return Err(OperationErrors::InputStringIsEmpty);
        }
        value.push_str(input);
        Ok(value.to_string())
    }

    fn cut(value: &mut String, amount: usize) -> Result<String,OperationErrors>{
        if amount == 0  {
            return Err(OperationErrors::CannotRemoveZeroCharacters);
        }
        if amount > value.len(){
            return Err(OperationErrors::AmountLargerThenString);
        }
     
        value.truncate(value.len() - amount);
        Ok(value.to_string())
    }

    
}

pub struct CommandProcessor<T> {
    commands: Vec<Operations>,
    current_position: usize,
    data: T
}

impl <T> CommandProcessor<T> {
    pub fn new(data: T) -> CommandProcessor<T>{
        CommandProcessor{
            commands: Vec::new(),
            current_position: 0,
            data
        }
    }
}

impl Commands<u32> for CommandProcessor<u32> {
    fn execute(&mut self, operation: Operations) -> Result<(), CommandErrors> {

        if self.current_position < self.commands.len(){
            self.commands.truncate(self.current_position);
        }

        match operation {
            Operations::Increment(amount) => {
                let result = Operations::increase(&mut self.data, amount)?;
                self.data = result;
                self.commands.push(operation);
                self.current_position += 1;
                Ok(())
            },
            Operations::Decrement(amount ) => {
                let result = Operations::decrease(&mut self.data, amount)?;
                self.data = result;
                self.commands.push(operation);
                self.current_position += 1;
                Ok(())
            },
            _ => {
                return Err(CommandErrors::InvalidOperationTypeOnData);
            }
        }
    }
    fn undo(&mut self,) -> Result<(), CommandErrors> {
            if self.current_position == 0 {
                return Err(CommandErrors::NothingToUndo);
            }
            let last_operation = &self.commands[self.current_position - 1];
            match last_operation {
                Operations::Increment(amount) => {
                    let result = Operations::decrease(&mut self.data, *amount)?;
                    self.data = result;
                    self.current_position -= 1;
                    Ok(())
                },
                Operations::Decrement(amount) => {
                    let result = Operations::increase(&mut self.data, *amount)?;
                    self.data = result;
                    self.current_position -= 1;
                    Ok(())
                },
                _ => {
                    Err(CommandErrors::InvalidOperationTypeOnData)
                }
            }
        }
    fn redo(&mut self,) -> Result<(), CommandErrors> {
        if self.current_position == 0  {
            return Err(CommandErrors::NothingToRedo)
        }
        let last_operation = &self.commands[self.current_position];
        match last_operation {
            Operations::Increment(amount) => {
                let result = Operations::increase(&mut self.data, *amount)?;
                self.data = result;
                self.current_position += 1;
                Ok(())
            },
            Operations::Decrement(amount) => {
                let result = Operations::decrease(&mut self.data, *amount)?;
                self.data = result;
                self.current_position += 1;
                Ok(())
            }
            _ => {
                return Err(CommandErrors::InvalidOperationTypeOnData);
            }
        }
    }
}

impl Commands<String> for CommandProcessor<String> {
    fn execute(&mut self, operation: Operations) -> Result<(), CommandErrors> {
        
    }
}
