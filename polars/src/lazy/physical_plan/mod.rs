pub mod executors;
pub mod expressions;
pub mod planner;

use crate::{lazy::prelude::*, prelude::*};
use arrow::datatypes::SchemaRef;
use std::fmt::Debug;
use std::rc::Rc;

pub enum ExprVal {
    Series(Series),
    Column(Vec<String>),
}

pub trait PhysicalPlanner {
    fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> Result<Rc<dyn Executor>>;
}

// Executor are the executors of the physical plan and produce DataFrames. They
// combine physical expressions, which produce Series.

/// Executors will evaluate physical expressions and collect them in a DataFrame.
pub trait Executor: Debug {
    fn schema(&self) -> SchemaRef {
        todo!()
    }
    fn execute(&self) -> Result<DataFrame>;
}

/// Take a DataFrame and evaluate the expressions.
/// Implement this for Column, lt, eq, etc
pub trait PhysicalExpr: Debug {
    fn data_type(&self, _input_schema: &Schema) -> Result<ArrowDataType> {
        unimplemented!()
    }
    /// Take a DataFrame and evaluate the expression.
    fn evaluate(&self, df: &DataFrame) -> Result<Series>;

    /// Get the output field of this expr
    fn to_field(&self, input_schema: &Schema) -> Result<Field>;

    fn as_agg_expr(&self) -> Result<&dyn AggPhysicalExpr> {
        Err(PolarsError::Other("not an agg expression".into()))
    }
}

pub trait AggPhysicalExpr {
    fn evaluate(&self, df: &DataFrame, groups: &[(usize, Vec<usize>)]) -> Result<Series>;
}
