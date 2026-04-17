use crate::errors::Error;
use crate::task::benchmark::{BenchmarkKind, Benchmarkable};
use crate::task::executable::{Executable, RunMode};
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use crate::tasks::TaskLogicError;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: usize,
    pub balance: Arc<Mutex<i64>>,
}

#[derive(Debug)]
pub struct Bank {
    accounts: Vec<Account>,
    accounts_amount: usize,
    initial_total: i64,
}

const THREAD_AMOUNT: usize = 5000;

impl Bank {
    pub fn new(accounts_amount: usize) -> Self {
        Self {
            accounts: vec![],
            accounts_amount,
            initial_total: 0,
        }
    }

    // Calculates the total money across all accounts to verify data integrity
    fn calculate_total_balance(&self) -> Result<i64, Error> {
        let mut total = 0;
        for account in &self.accounts {
            total += *account
                .balance
                .lock()
                .map_err(|_| TaskLogicError::MutexPoisoned)?;
        }

        Ok(total)
    }

    // Generates a deterministic pseudo-random index for transferring
    fn get_account_pair(&self, iteration: usize) -> (Account, Account) {
        let first_id = (iteration * 17) % self.accounts_amount;
        let second_id = (iteration * 31) % self.accounts_amount;

        let second_id = if first_id == second_id {
            (second_id + 1) % self.accounts_amount
        } else {
            second_id
        };

        let first_account = self.accounts[first_id].clone();
        let second_account = self.accounts[second_id].clone();

        (first_account, second_account)
    }
}

impl Reportable for Bank {
    fn name(&self) -> &'static str {
        "Bank Transactions"
    }
}

impl Measurable for Bank {}

impl Benchmarkable for Bank {
    fn benchmarks(&self) -> Vec<BenchmarkKind> {
        vec![
            BenchmarkKind::RaceCondition,
            BenchmarkKind::Deadlock,
            BenchmarkKind::MutexFixed,
        ]
    }
}

impl Manageable for Bank {
    fn setup(&mut self) -> Result<(), Error> {
        self.accounts.clear();
        self.initial_total = 0;

        for id in 0..self.accounts_amount {
            // Assigning a pseudo-random initial balance between 100 and 1100
            let balance = 100 + (id * 37 % 1000) as i64;
            self.initial_total += balance;

            self.accounts.push(Account {
                id,
                balance: Arc::new(Mutex::new(balance)),
            });
        }

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        self.accounts.clear();
        Ok(())
    }
}

impl Executable for Bank {
    fn supported_modes(&self) -> Vec<RunMode> {
        vec![
            RunMode::RaceCondition,
            RunMode::Deadlock,
            RunMode::MutexFixed,
        ]
    }

    fn run(&self, kind: BenchmarkKind) -> Result<(), Error> {
        match kind {
            BenchmarkKind::RaceCondition => self.run_race_condition()?,
            BenchmarkKind::Deadlock => self.run_deadlock()?,
            BenchmarkKind::MutexFixed => self.run_mutex_fixed()?,
            _ => unreachable!(),
        };

        // Skip total balance validation for Deadlock because background threads
        // will hold the locks forever, causing the main thread to freeze here.
        if !matches!(kind, BenchmarkKind::Deadlock) {
            let final_total = self.calculate_total_balance()?;
            log::info!(
                "[{}] Initial Funds: {}. Final Funds: {}. Difference: {}",
                kind,
                self.initial_total,
                final_total,
                final_total - self.initial_total
            );
        }

        Ok(())
    }
}

impl Bank {
    fn run_race_condition(&self) -> Result<(), Error> {
        let mut handles = vec![];

        for i in 0..THREAD_AMOUNT {
            let (from, to) = self.get_account_pair(i);
            let transaction_diff = 10;

            handles.push(thread::spawn(move || -> Result<(), TaskLogicError> {
                // Simulating a data race by dropping the lock between read and write
                let balance_from = from
                    .balance
                    .lock()
                    .map_err(|_| TaskLogicError::MutexPoisoned)?;
                if *balance_from >= transaction_diff {
                    let old_from = *balance_from;
                    drop(balance_from);

                    let balance_to = to
                        .balance
                        .lock()
                        .map_err(|_| TaskLogicError::MutexPoisoned)?;
                    let old_to = *balance_to;
                    drop(balance_to);

                    thread::yield_now();

                    *from
                        .balance
                        .lock()
                        .map_err(|_| TaskLogicError::MutexPoisoned)? =
                        old_from - transaction_diff;
                    *to.balance
                        .lock()
                        .map_err(|_| TaskLogicError::MutexPoisoned)? =
                        old_to + transaction_diff;
                }

                Ok(())
            }));
        }

        // If there are errors, return them
        for handle in handles {
            let thread_logic_result = handle
                .join()
                .map_err(|_| TaskLogicError::JoinThreadFailed)?;
            thread_logic_result?;
        }

        Ok(())
    }

    fn run_deadlock(&self) -> Result<(), Error> {
        let (tx, rx) = std::sync::mpsc::channel();

        let accounts = self.accounts.clone();
        let accounts_amount = self.accounts_amount;

        // Executing the deadlock scenario inside an isolated thread to prevent permanent freeze
        thread::spawn(move || -> Result<(), TaskLogicError> {
            let mut handles = vec![];

            for i in 0..THREAD_AMOUNT {
                let from_id = (i * 17) % accounts_amount;
                let mut to_id = (i * 31) % accounts_amount;
                if from_id == to_id {
                    to_id = (to_id + 1) % accounts_amount;
                }

                let account_from = accounts[from_id].clone();
                let account_to = accounts[to_id].clone();

                handles.push(thread::spawn(move || -> Result<(), TaskLogicError> {
                    let _lock1 = account_from
                        .balance
                        .lock()
                        .map_err(|_| TaskLogicError::MutexPoisoned)?;

                    // Forcing a context switch to heavily increase deadlock probability
                    thread::sleep(Duration::from_millis(1));

                    let _lock2 = account_to
                        .balance
                        .lock()
                        .map_err(|_| TaskLogicError::MutexPoisoned)?;

                    Ok(())
                }));
            }

            for handle in handles {
                let thread_logic_result = handle
                    .join()
                    .map_err(|_| TaskLogicError::JoinThreadFailed)?;
                thread_logic_result?;
            }

            let _ = tx.send(());

            Ok(())
        });

        // If the threads do not finish within 2 seconds, we successfully demonstrated a deadlock
        if rx.recv_timeout(Duration::from_secs(2)).is_err() {
            log::warn!(
                "Deadlock successfully detected! Execution halted to prevent permanent freeze."
            );
        }

        Ok(())
    }

    fn run_mutex_fixed(&self) -> Result<(), Error> {
        let mut handles = vec![];

        for i in 0..THREAD_AMOUNT {
            let (from, to) = self.get_account_pair(i);
            let amount = 10;

            handles.push(thread::spawn(move || -> Result<(), TaskLogicError> {
                // Global lock ordering avoids circular waits.
                // We lock the account with the smaller ID first, then the larger ID.
                let (from, to) = if from.id < to.id {
                    (from, to)
                } else {
                    (to, from)
                };

                let mut guard_from = from
                    .balance
                    .lock()
                    .map_err(|_| TaskLogicError::MutexPoisoned)?;
                let mut guard_to = to
                    .balance
                    .lock()
                    .map_err(|_| TaskLogicError::MutexPoisoned)?;

                if *guard_from >= amount {
                    *guard_from -= amount;
                    *guard_to += amount;
                }

                Ok(())
            }));
        }

        for handle in handles {
            let thread_logic_result = handle
                .join()
                .map_err(|_| TaskLogicError::JoinThreadFailed)?;
            thread_logic_result?;
        }

        Ok(())
    }
}
