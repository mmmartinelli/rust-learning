pub(crate) fn structs() {
    checking_account();
}

struct CheckingAccount {
    holder_name: Holder,
    balance: f64
}

impl CheckingAccount {
    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }
}

struct Holder {
    name: String,
    last_name: String
}

fn checking_account() {
    let mut ca1: CheckingAccount = CheckingAccount {
        holder_name: Holder {
            name: String::from("Gary"), 
            last_name: String::from("Blue") 
        },
        balance: 1000.00
    };

    let mut ca2: CheckingAccount = CheckingAccount {
        holder_name: Holder {
            name: String::from("Ash"), 
            last_name: String::from("Red")
        },
        balance: 259000.00
    };

    println!("[Checking account #1] Holder: {} {}, Balance: U$ {}",
        ca1.holder_name.name,
        ca1.holder_name.last_name,
        ca1.balance
    );

    println!("[Checking account #2] Holder: {} {}, Balance: U$ {}",
        ca2.holder_name.name,
        ca2.holder_name.last_name,
        ca2.balance
    );

    ca1.withdraw(50.0);
    ca2.withdraw(250.0);

    println!("[Checking account #1] Holder: {} {}, Balance: U$ {}",
        ca1.holder_name.name,
        ca1.holder_name.last_name,
        ca1.balance
    );

    println!("[Checking account #2] Holder: {} {}, Balance: U$ {}",
        ca2.holder_name.name,
        ca2.holder_name.last_name,
        ca2.balance
    );
}