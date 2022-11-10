use super::expense::*;

pub struct Ledger {
    pub expenses: Vec<Expense>,
    pub balance: f64
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            balance: 0.0,
            expenses: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        self.log_expense(amount, ExpenseFlow::In);
    }

    pub fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
        self.log_expense(amount, ExpenseFlow::Out);
    }

    fn log_expense(&mut self, amount: f64, flow: ExpenseFlow) {
        let expense = Expense::new(amount, flow);
        self.expenses.push(expense);
    }
}
