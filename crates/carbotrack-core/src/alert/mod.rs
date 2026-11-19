use crate::carbon::EmissionBreakdown;

#[derive(Debug, Clone, PartialEq)]
pub struct BudgetAlert {
    pub budget: f64,
    pub current: f64,
    pub exceeded: bool,
}

pub fn evaluate(budget: f64, emissions: EmissionBreakdown) -> BudgetAlert {
    let current = emissions.total();
    BudgetAlert {
        budget,
        current,
        exceeded: current > budget,
    }
}
