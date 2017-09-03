//! Access the notifications portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

// Declaration of types representing the various items under users
new_type!(
    Notifications
);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Notifications  = "notifications"
    @Notifications
        => Executor
);

impl_macro!(
    @Notifications
        |        
        |-> execute
);
