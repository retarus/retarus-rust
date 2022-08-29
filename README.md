## Retarus-Rust SDK
The offical rust SDK provided by retaurs to contact the messaging services.

## Installation
Just add the crate to your project:

```toml
[dependencies]
retarus = "0.1"
```



## Usage
The Python-SDK implements different services that are offered by retarus. So in our example we will use the fax client. Before we can send a fax we need to create a new instance of the FaxClient. The SDK offers a common rust pattern, a builder for the sdk.

```rust
use retarus::fax::client::{FaxClient};
use retarus::common::region::{Region};

let client = FaxClient::builder()
    .set_credentiale(Credentails::new("your_user_id", "your_password"))
    .set_customer_number("customer_number".to_string())
    .set_region(Region::Europe)
    .build();
```

Now we are ready to go, lets create a instance of a job and send it.

```rust
let job = ....
// some stuff to create a job
let res:FaxJobResponse = client.send_job(None, job).await;
println!("Job send, report: {}", res)
```
Now we just send a fax without using a faxing machine.

## Sync / Async
The whole sdk is written in asynchronous and should be used this way. If you need to use the sdk in a synchronous matter you can easily use the blocking method that is offered by the sdk. It can be used on each service like this:

```rust
use retarus::common::transport::blocking;
use retarus::common::creds::Credentials;
use retarus::sms::client::{SmsClient};
use retarus::common::uri::Region;

fn main() {
let sdk = SmsClient::builder()
        .set_region(Region::Europe)
        .set_credentiale(Credentials::from_env().expect("You need to export your credentials"))
        .build();
let job = SmsJob::builder()
        .add_message("Hallo Welt".to_string(), vec!["490000000000".to_string()])
        .build();

// This is the way you need to use the blocking method, to await the result in the current thread.
let res = blocking(sdk.send_sms(job)).expect("Error while sending sms job");
}
```

## Region
The SDK also offers a simply way to select a region where your jobs should be processed. By default the SDK will use the Europe region.
To Configure it simply use the set_region function in the builder for your client, just like above. But not each service is available to beeing processed in different [regions](retarus/src/general/uri.rs)

## Examples
Each service provides a small variety of example to get a better understanding how to use their functionality. The examples can be found in the examples directory sorted after each services.