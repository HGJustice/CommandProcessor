use commandProcessor::errors::CommandErrors; 
use commandProcessor::types::{CommandProcessor, Operations};

#[cfg(test)]
mod tests {

    use commandProcessor::types::Commands;

    use super::*; 

    #[test]
    fn test_number_operations(){
        let mut value: u32 = 5;
        let result = Operations::increase(&mut value, 3).unwrap();
        assert_eq!(result, 8);

        let mut value: u32 = 5;
        assert!(matches!(Operations::increase(&mut value, 0), 
            Err(CommandErrors::CannotIncreaseByZero)));
        
        let mut value = u32::MAX;
        assert!(matches!(Operations::increase(&mut value, 1),
            Err(CommandErrors::IntegerOverflowError)));

        let mut value:u32 = 5;
        let result = Operations::decrease(&mut value, 2).unwrap();
        assert_eq!(result, 3);

        let mut value: u32 = 5;
        assert!(matches!(Operations::decrease(&mut value, 0),
            Err(CommandErrors::CannotDecreaseByZero)));
        
        let mut value = u32::MIN;
        assert!(matches!(Operations::decrease(&mut value, 1),
            Err(CommandErrors::IntergerUnderflowError)));
    }

    #[test]
    fn test_string_operations(){
        let mut value = String::from("Hello");
        let result = Operations::append(&mut value, "world").unwrap();
        assert_eq!(result, "Helloworld");

        let mut value = String::from("Hello");
        assert!(matches!(Operations::append(&mut value, ""),
            Err(CommandErrors::InputStringIsEmpty)));

        let mut value = String::from("Hello World");
        let (result, removed) = Operations::cut(&mut value, 6).unwrap();
        assert_eq!(result, "Hello");
        assert_eq!(removed, " World");

        let mut value = String::from("Hello");
        assert!(matches!(Operations::cut(&mut value, 0), 
            Err(CommandErrors::CannotRemoveZeroCharacters)));

        let mut value = String::from("Hello");
        assert!(matches!(Operations::cut(&mut value, 6), 
            Err(CommandErrors::AmountLargerThenString)))
    }

    #[test]
    fn test_command_processor_basics(){
        let processor = CommandProcessor::new(10);
        assert!(processor.data == 10);
        assert!(processor.commands.is_empty());
        assert!(processor.current_position == 0);

        let mut processor = CommandProcessor::new(5);
        processor.execute(Operations::Increment(3)).unwrap();
        assert!(processor.data == 8);
        assert!(processor.current_position == 1);
        assert_eq!(processor.commands[0], Operations::Increment(3));

        let mut processor = CommandProcessor::new(String::from("hello"));
        processor.execute(Operations::Append(String::from(" world"))).unwrap();
        assert!(processor.data == "hello world".to_string());
        assert!(processor.current_position == 1);
        assert_eq!(processor.commands[0], Operations::Append(" world".to_string()));

        let mut processor = CommandProcessor::new(5u32);
        assert!(matches!(
            processor.execute(Operations::Append(String::from("test"))),
            Err(CommandErrors::InvalidOperationTypeOnData)
        ));
    }

    #[test]
    fn test_undo_redo_numbers() {
        let mut processor = CommandProcessor::new(5);
        processor.execute(Operations::Increment(3)).unwrap();
        assert_eq!(processor.data, 8);

        processor.undo().unwrap();
        assert_eq!(processor.data, 5);

        processor.redo().unwrap();
        assert_eq!(processor.data, 8);

        processor.execute(Operations::Decrement(3)).unwrap();
        processor.execute(Operations::Increment(5)).unwrap();
        processor.undo().unwrap();
        processor.undo().unwrap();
        assert_eq!(processor.data, 8);
        assert_eq!(processor.current_position, 1);

        processor.undo().unwrap();
        assert!(matches!(processor.undo(), Err(CommandErrors::NothingToUndo)));

        processor.execute(Operations::Increment(2)).unwrap();
        assert!(matches!(processor.redo(), Err(CommandErrors::NothingToRedo)));
    }

    #[test]
    fn test_string_undo_redo() {
        let mut processor = CommandProcessor::new(String::from("Hello"));
        processor.execute(Operations::Append(String::from(" World"))).unwrap();
        processor.undo().unwrap();
        processor.redo().unwrap();
        assert_eq!(processor.data, "Hello World".to_string());

        processor.execute(Operations::Append(String::from("hi"))).unwrap();
        processor.execute(Operations::Append(String::from("hi"))).unwrap();
        processor.undo().unwrap();
        processor.undo().unwrap();
        assert_eq!(processor.data, "Hello World".to_string());
        assert_eq!(processor.current_position, 1);

        processor.undo().unwrap();
        assert!(matches!(processor.undo(), Err(CommandErrors::NothingToUndo)));

        processor.execute(Operations::Append(String::from("hi"))).unwrap();
        assert!(matches!(processor.redo(), Err(CommandErrors::NothingToRedo)));
    }
    
    #[test]
    fn test_command_history(){
        let mut processor = CommandProcessor::new(5);
        processor.execute(Operations::Increment(3)).unwrap();
        processor.execute(Operations::Increment(4)).unwrap();
        processor.execute(Operations::Increment(5)).unwrap();
        processor.undo().unwrap();
        processor.undo().unwrap();
        assert_eq!(processor.current_position, 1);

        processor.execute(Operations::Decrement(2)).unwrap();
        assert_eq!(processor.commands.len(), 2);

        processor.undo().unwrap();
        processor.redo().unwrap();
        processor.undo().unwrap();
        assert_eq!(processor.current_position, 1);
        assert_eq!(processor.data, 8);
    }

}