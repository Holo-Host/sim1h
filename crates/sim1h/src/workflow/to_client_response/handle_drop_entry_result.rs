use crate::trace::LogContext;
use crate::trace::tracer;

// result of no-op is no-op
pub fn handle_drop_entry_result(log_context: &LogContext) {
    tracer(&log_context, &format!("handle_drop_entry_result"));
}