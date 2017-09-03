extern crate github_rs;
use github_rs::client::Github;

fn main() {
    let client = Github::new("93e76f6827a0b30d9dfd5bf78cc9a02b1f4e79d1");
    let notifications = client.get()
        .notifications()
        .execute();
    match notifications {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}
