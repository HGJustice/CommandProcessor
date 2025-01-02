pub enum CommandErrors {
    ExecuteError,
    UndoError,
    RedoError,
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