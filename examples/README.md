## Examples

This folder contains a small set of applications which should visualize how the sms sdk can be used to dispatch messages and fulfill your business needs.

### Examples

name | description | run command |
--- | --- | --- |
[fax_daemon](https://github.com/retarus/retarus-rust/tree/master/examples/fax_daemon) | Example application which sends faxes from a out directory, processes it and stores the according reports in the in folder | cd fax_daemon && cargo run 
[send_fax_async](https://github.com/retarus/retarus-rust/tree/master/examples/send_fax_async.rs) |  Send a fax using the async api provided by this sdk | cargo run --example send_fax_async | 
[send_sms_async](https://github.com/retarus/retarus-rust/tree/master/examples/send_sms_async.rs) | Shows how the smsjob is build and dispatched via the async client provided by the sdk | cargo run --example send_sms_async |
[send_sms_sync](https://github.com/retarus/retarus-rust/tree/master/examples/send_sms_sync.rs) | Shows how the smsjob is build and dispatched the job utilizing the blocking function to run it in sync| cargo run --example send_sms_sync |
[upload_webexpress_async](https://github.com/retarus/retarus-rust/tree/master/examples/upload_webexpress_async.rs) | Shows how to upload a file to webexpress | cargo run --example upload_webexpress_async |

### Credentials
All the example applications still require credentials to authorize your request on the retarus server. So for that the sdk uses environment variables. In this case you can simply export you credentials in your current terminal like this:
```bash
export retarus_userid="your_retarus_userid"
```

Or you could use an ".env" file which contains your credentials and other information with the same variable names as they are called in the examples.
If you are using the .env file you need to add following line to the example and install following package:


```bash
cargo add dotenv
```


```rust
use dotenv::dotenv;
use std::env;
dotenv().ok();
```


To find out which variables are needed you can search for the following statements in the examples:
```rust
use retarus::common::creds::Credentials;
use std::env;

let username = std::env::var("retarus_userid").unwrap().as_str();
let password = std::env::var("retarus_fax_password").unwrap().as_str();;
let creds = Credentials::new(username, password);
```



