use crate::backend::Backend;
use crate::query_builder::QueryFragment;

/// Describes how a query needs to be executed by a connection implementation
///
/// Most queries can be executed by sending a single SQL statement to the database,
/// which is represented by [`SingleQueryStrategy`]. Some queries require
/// special handling for certain backends. For example batch inserts with
/// SQLite might need to be split into several statements if they contain
/// `DEFAULT` values. Connection implementations can use the `Strategy` type
/// to dispatch to a matching execution path.
///
/// This trait is only useful for crates that provide their own connection
/// implementations which cannot rely on diesel's `RunQueryDsl` implementation.
pub trait QueryExecutionStrategy<DB: Backend> {
    /// A marker type describing how to execute this query
    type Strategy;
}

/// The default execution strategy: the query is a valid [`QueryFragment`]
/// for the given backend and can be executed as a single statement
#[derive(Debug, Clone, Copy)]
pub struct SingleQueryStrategy;

impl<T, DB> QueryExecutionStrategy<DB> for T
where
    DB: Backend,
    T: QueryFragment<DB>,
{
    type Strategy = SingleQueryStrategy;
}
