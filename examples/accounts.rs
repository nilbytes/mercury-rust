use mercury_rust::client::Client;
use mercury_rust::resources::accounts::Account;
use mercury_rust::resources::ListData;
use std::env;

#[tokio::main]
async fn main() -> mercury_rust::Result<()> {

    let secret_key = env::var("API_KEY").expect("Missing 'API_KEY'.");
    let client = Client::new(secret_key);

    let accounts = Account::list(&client).await?;

    if let ListData::<Account>::Accounts( ref list ) = accounts.data {

        for account in list.iter() {
            println!("Account: {}", account.name);
            println!("Available Balance: {}", account.available_balance);
            println!("Current Balance: {}", account.current_balance);
            println!("Account Type: {:?}", account.kind);
            println!();
        }

    }

    Ok(())
}