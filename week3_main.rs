Перечисление типов операций в Solana
enum TransactionType {
    Transfer,
    Stake,
}

Структура аккаунта Solana
struct SolanaAccount {
    pubkey: String,
    balance_lamports: u64,
}

impl SolanaAccount {
    // Метод конвертации Lamports в SOL (1 SOL = 1,000,000,000 Lamports)
    fn get_balance_in_sol(&self) -> f64 {
        self.balance_lamports as f64 / 1_000_000_000.0
    }
}

fn main() {
    println!("=== Solana CLI Helpers (Rust) ===");

    let my_account = SolanaAccount {
        pubkey: String::from("5Rh8M2XsKBJJTC1UdEFxPZRUAqC1UaoSM87q6s9ovVfJ"),
        balance_lamports: 999_029_186,
    };

    let tx_type = TransactionType::Transfer;

    println!("Account Public Key: {}", my_account.pubkey);
    println!("Balance in Lamports: {}", my_account.balance_lamports);
    println!("Balance in SOL: {} SOL", my_account.get_balance_in_sol());

    match tx_type {
        TransactionType::Transfer => println!("Operation Type: SOL Transfer"),
        TransactionType::Stake => println!("Operation Type: Staking"),
    }
}
