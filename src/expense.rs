pub struct Expense {
    pub amount: f64,
    pub flow: ExpenseFlow
}

impl Expense {
    pub fn new(amount: f64, flow: ExpenseFlow) -> Self {
        Self {
            amount,
            flow
        }
    }
}

pub enum ExpenseFlow {
    In,
    Out
}
