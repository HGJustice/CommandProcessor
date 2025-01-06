#[derive(Debug)]
pub enum CommandErrors {
    IntegerOverflowError,
    IntergerUnderflowError,
    CannotIncreaseByZero,
    CannotDecreaseByZero,
    InputStringIsEmpty,
    CannotRemoveZeroCharacters,
    AmountLargerThenString,
    InvalidOperationTypeOnData,
    NothingToUndo,
    NothingToRedo,
}

pub enum OperationErrors {
    IntegerOverflowError,
    IntergerUnderflowError,
    CannotIncreaseByZero,
    CannotDecreaseByZero,
    InputStringIsEmpty,
    CannotRemoveZeroCharacters,
    AmountLargerThenString,
}