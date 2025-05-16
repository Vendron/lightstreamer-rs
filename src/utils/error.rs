use std::error::Error;
use std::fmt;

/// Exception thrown when an illegal or inappropriate argument is passed to a method.
///
/// This exception indicates that a method has been passed an illegal or inappropriate argument.
/// It is similar to Java's IllegalArgumentException.
#[derive(Debug)]
pub struct IllegalArgumentException(String);

impl IllegalArgumentException {
    /// Creates a new IllegalArgumentException with the specified detail message.
    ///
    /// # Arguments
    /// * `msg` - The detail message explaining the reason for the exception
    ///
    /// # Returns
    /// A new IllegalArgumentException instance with the specified message
    pub fn new(msg: &str) -> IllegalArgumentException {
        IllegalArgumentException(msg.to_string())
    }
}

impl fmt::Display for IllegalArgumentException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IllegalArgumentException {
    fn description(&self) -> &str {
        &self.0
    }
}

/// Exception thrown when a method is invoked at an illegal or inappropriate time.
///
/// This exception indicates that a method has been invoked at an inappropriate time
/// or that the object is in an inappropriate state for the requested operation.
/// It is similar to Java's IllegalStateException.
#[derive(Debug)]
pub struct IllegalStateException {
    details: String,
}

impl IllegalStateException {
    /// Creates a new IllegalStateException with the specified detail message.
    ///
    /// # Arguments
    /// * `msg` - The detail message explaining the reason for the exception
    ///
    /// # Returns
    /// A new IllegalStateException instance with the specified message
    pub fn new(msg: &str) -> IllegalStateException {
        IllegalStateException {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for IllegalStateException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for IllegalStateException {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    // Test trait implementations for IllegalArgumentException
    #[test]
    fn test_illegal_argument_exception_creation() {
        // Create exception with a test message
        let error_msg = "Test error message";
        let exception = IllegalArgumentException::new(error_msg);

        // Test that the message is stored correctly
        assert_eq!(exception.0, error_msg);

        // Test Debug implementation
        let debug_output = format!("{:?}", exception);
        assert!(debug_output.contains(error_msg));

        // Test Display implementation
        let display_output = format!("{}", exception);
        assert_eq!(display_output, error_msg);

        // Test Error trait implementation
        let description = exception.to_string();
        assert_eq!(description, error_msg);
    }

    // Test trait implementations for IllegalStateException
    #[test]
    fn test_illegal_state_exception_creation() {
        // Create exception with a test message
        let error_msg = "Test state error";
        let exception = IllegalStateException::new(error_msg);

        // Test that the message is stored correctly
        assert_eq!(exception.details, error_msg);

        // Test Debug implementation
        let debug_output = format!("{:?}", exception);
        assert!(debug_output.contains(error_msg));

        // Test Display implementation
        let display_output = format!("{}", exception);
        assert_eq!(display_output, error_msg);

        // Test Error trait implementation
        let description = exception.to_string();
        assert_eq!(description, error_msg);
    }

    // Test error propagation with ? operator
    #[test]
    fn test_error_propagation() {
        // Helper function that returns IllegalArgumentException
        fn function_that_fails() -> Result<(), IllegalArgumentException> {
            Err(IllegalArgumentException::new("Test propagation"))
        }

        // Function that propagates the error
        fn propagate_error() -> Result<(), Box<dyn Error>> {
            function_that_fails()?;
            Ok(())
        }

        // Test that the error is propagated correctly
        let result = propagate_error();
        assert!(result.is_err());
        if let Err(boxed_error) = result {
            let error_message = boxed_error.to_string();
            assert_eq!(error_message, "Test propagation");
        }
    }

    // Test conversion between error types
    #[test]
    fn test_error_conversion() {
        // Create an IllegalArgumentException
        let arg_exception = IllegalArgumentException::new("Invalid argument");

        // Convert to a Box<dyn Error>
        let boxed_error: Box<dyn Error> = Box::new(arg_exception);

        // Check that the error message is preserved
        assert_eq!(boxed_error.to_string(), "Invalid argument");

        // Create an IllegalStateException
        let state_exception = IllegalStateException::new("Invalid state");

        // Convert to a Box<dyn Error>
        let boxed_error: Box<dyn Error> = Box::new(state_exception);

        // Check that the error message is preserved
        assert_eq!(boxed_error.to_string(), "Invalid state");
    }

    // Test error as a return type
    #[test]
    fn test_error_as_return_type() {
        // Function that returns different error types based on input
        fn may_fail(value: i32) -> Result<(), Box<dyn Error>> {
            if value < 0 {
                Err(Box::new(IllegalArgumentException::new(
                    "Value cannot be negative",
                )))
            } else if value > 100 {
                Err(Box::new(IllegalStateException::new("Value too large")))
            } else {
                Ok(())
            }
        }

        // Test negative value (should return IllegalArgumentException)
        let result = may_fail(-10);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Value cannot be negative");
        }

        // Test large value (should return IllegalStateException)
        let result = may_fail(150);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Value too large");
        }

        // Test valid value (should return Ok)
        let result = may_fail(50);
        assert!(result.is_ok());
    }

    // Test using errors with the std::error::Error trait
    #[test]
    fn test_error_trait_methods() {
        // Test with IllegalArgumentException
        let arg_exception = IllegalArgumentException::new("Test error");
        let error: &dyn Error = &arg_exception;

        // Test source method (should be None since we don't have a cause)
        assert!(error.source().is_none());

        // Test description method directly
        assert_eq!(arg_exception.to_string(), "Test error");

        // Test with IllegalStateException
        let state_exception = IllegalStateException::new("Test state error");
        let error: &dyn Error = &state_exception;

        // Test source method (should be None since we don't have a cause)
        assert!(error.source().is_none());

        // Test description method directly
        assert_eq!(state_exception.to_string(), "Test state error");
    }

    // Test Debug formatting for both error types
    #[test]
    fn test_debug_formatting() {
        // Test IllegalArgumentException
        let arg_exception = IllegalArgumentException::new("Test arg error");
        let debug_str = format!("{:?}", arg_exception);
        assert!(debug_str.contains("IllegalArgumentException"));
        assert!(debug_str.contains("Test arg error"));

        // Test IllegalStateException
        let state_exception = IllegalStateException::new("Test state error");
        let debug_str = format!("{:?}", state_exception);
        assert!(debug_str.contains("IllegalStateException"));
        assert!(debug_str.contains("Test state error"));
    }
}
