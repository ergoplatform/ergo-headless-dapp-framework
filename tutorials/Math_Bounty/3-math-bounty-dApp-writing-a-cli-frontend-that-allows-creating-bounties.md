# 3. Math Bounty dApp - Writing A CLI Frontend That Allows Creating Bounties

In the last two tutorials we created the Math Bounty headless dApp which provides us with a pure interface for interacting with our smart contract protocol. In this tutorial we are going to begin using our headless dApp to create a textual front-end for it as a CLI app.

The vast majority of the design patterns and code we write will be equally as applicable to GUI-based front-ends as well, however to keep this tutorial concise we are going to be focused on creating a CLI interface instead. (This tutorial series targets coding everything in Rust, but the Ergo HDF/headless dApps built with it are usable with other languages as well with little to no extra work. This can be done by compiling to WASM using wasm-pack for example, so other languages such as JS can take advantage and utilize these exact same design patterns.)


## Creating The Project

We will create a new rust project (best to keep it in the same outer folder as your headless dApp) for our Math Bounty CLI app:

```
cargo new math-bounty-cli
```

In your new project folder edit the `Cargo.toml` and add your `math-bounty-headless` as a dependency, as well as the `ergo-node-interface` lib and `reqwest`.

```rust
[dependencies]
math-bounty-headless     = {path = "../math-bounty-headless"}
ergo-node-interface      = "0.2.2"
reqwest                 = {version ="0.10.9", features = ["blocking"]}
```

The `ergo-node-interface` is a Rust crate(library) which provides with all the functions you will need to interface with an Ergo node wallet so that you can do things such as acquiring the user's address, or asking the node wallet to sign and submit a generated `UnsignedTransaction`.

The `reqwest` library is a library we will be using for issuing GET requests. Feel free to use any other Rust library that fulfills said task.


## Setting Up And Using The `NodeInterface`

Continuing to your `main.rs` we will start by importing everything from your `math-bounty-headless` and the `ergo-node-interface` lib (plus the `get` function from `reqwest`).

```rust
use math_bounty_headless::*;
use ergo_node_interface::*;
use reqwest::blocking::get;
```

Next we will create a new Ergo `NodeInterface` instance. This will allow us to interact with an Ergo Node via Rust. Do note, a user of the CLI app will need to have an unlocked Ergo Node wallet available in order for the CLI dApp to function.

We will be using `acquire_node_interface_from_local_config` from the Ergo Node Interface library to simplify the process of creating an `NodeInterface`. In short from the documentation:
```rust
/// A ease-of-use function which attempts to acquire a `NodeInterface`
/// from a local file. If the file does not exist, it generates a new
/// config file, tells the user to edit the config file, and then closes
/// the running application
/// This is useful for CLI applications, however should not be used by
/// GUI-based applications.
pub fn acquire_node_interface_from_local_config() -> NodeInterface;
```

As such on first run of our CLI application, the user will have a config file automatically generated for them, and be prompted to edit it with information about how to connect to their Ergo Node (ip/port/api_key). After that initial setup, the function will automatically generate a `NodeInterface` without any prompts, allowing the application to function normally.

```rust
fn main() {
    // Get a `NodeInterface`
    let node = acquire_node_interface_from_local_config();
}
```

Now that we have a `NodeInterface` which will query an Ergo Node wallet for us, we can use a couple methods to easily acquire the user's first address in their wallet, as well the current block height.

```rust
{
    // Get a `NodeInterface`
    let node = acquire_node_interface_from_local_config();
    // Get the current Ergo Blockchain block height
    let block_height = node.current_block_height().unwrap();
    // Get the first address in the user's wallet
    let user_address = node.wallet_addresses().unwrap()[0].clone();
}
```

And just like that we have all the information we need from the user's node wallet in 3 lines of code.


### Implement Argument Checking

Next we are going to implement argument checking for our CLI application. In our `main` function we will simply add this line to acquire the arguments that were submit by the user who ran our application.

```rust
    // Acquire CLI arguments
    let args: Vec<String> = std::env::args().collect();
```

We will do a single basic check that verifies the user is trying to issue the "Bootstrap Math Bounty Box" action, by using the `bounty` command.

```rust
if args.len() == 3 {
    // User wishes to submit nanoErgs to create a new `MathBountyBox`
    if args[1] == "bounty" {
        let bounty_amount_in_nano_ergs = args[2].parse::<u64>().unwrap();
        todo!();
    }
}
```

### Implementing The Submit Bounty CLI Logic

The CLI should allow a user to use the `bounty` command and provide an integer in order to build the "Bootstrap Math Bounty Box" Action using our headless dApp.

From the argument checking code block above, we will now be filling out the logic.

The first thing we will do is allow the user to submit a number of Ergs, rather than nanoErgs. This makes it much easier for our average user to understand how much they are spending. This is simple with the HDF:

```rust
// Taking user input as Ergs and converting to nanoErgs
let bounty_amount_in_nano_ergs = erg_to_nano_erg(args[2].parse::<f64>().unwrap());
```

As such, a user can now utilize our `bounty` command via: `./math-bounty-cli bounty 1.5`, and the 1.5 Ergs will be properly converted to nanoErgs. The reverse operation is also supported by the EDF via the `nano_erg_to_erg` function.

Continuing forward we will look at implementing the action itself. If you recall, these are the inputs that are required for the "Bootstrap Math Bounty Box" action:

```rust
bounty_amount_in_nano_ergs: u64,
ergs_box_for_bounty: ErgsBox,
current_height: u64,
transaction_fee: u64,
ergs_box_for_fee: ErgsBox,
user_address: String,
```

Currently we are missing the following inputs:
1. `transaction_fee`
2. `ergs_box_for_bounty`
3. `ergs_box_for_fee`

We can get the transaction fee out of the way as it is quite simple:

```rust
let tx_fee = 1000000;
```

Now we can begin working on the two `ErgsBox`es. Let's address the `ergs_box_for_bounty` first.

One of the great features of the HDF for front-end developers is that you have a streamlined method of acquiring boxes via `BoxSpec`s. This means that instead of having to manually figure our how to find these boxes as inputs for the headless dApp, you just focus on building valid `BoxSpec` structs, and then the HDF provides you with all of the methods you need to find said box(es).

Furthermore, the HDF provides a very effective modifiable `BoxSpec` interface. As we will see in this next code section below, we will modify the default `ErgsBox` `BoxSpec` so that it specifies the user's address & has at least `bounty_amount_in_nano_ergs`.

```rust
// Take the generalized `BoxSpec` from an `ErgsBox` and modify it
// for our use case. Specifically change the address to be our
// user's address, and change the value_range so that the box
// has enough to cover the bounty amount.
let ergs_box_for_bounty_spec = ErgsBox::box_spec()
    .modified_address(Some(user_address))
    .modified_value_range(Some(bounty_amount_in_nano_ergs..u64::MAX));
```

This specifies exactly what we require for our Action. We need a valid `ErgsBox` for our action which our user can spend and has enough Ergs inside of it to pay for the bounty amount.

Now that we've modified the `BoxSpec` to meet our current requirements, we can trivially acquire a list of boxes from an Ergo Explorer API that match said spec. To do this we will first use the `.explorer_endpoint` method together with a link to a public instance of the Ergo Explorer API. (Optimally you would deploy your own Ergo Explorer Backend/API to support your own dApp to ensure uptime/increase decentralization, but for our example we will use the public API for testing)

This method will then return a new url as a String, however with the correct endpoint in order to find boxes which match out spec. Thus once we have the new endpoint-appended url, we will issue a GET request and save the response body as a String in `get_response`.

```rust
let ergs_box_for_bounty_url = ergs_box_for_bounty_spec
    .explorer_endpoint("https://api.ergoplatform.com/api")
    .unwrap();
// Make a get request to the Ergo Explorer API endpoint
let get_response = get(&ergs_box_for_bounty_url).unwrap().text().unwrap();
```

This response will be a list of boxes as a json String which may or may not match our spec entirely. As such, we need to process this explorer response using the HDF to acquire the `ErgsBox`es which specifically match our modified spec. This is done easily enough via `ErgsBox::process_explorer_response_custom`. (Note, `process_explorer_response` is also available, however that would use the default `BoxSpec` of an `ErgsBox`. Because we modified the `BoxSpec` to match our requirements, we must use the `_custom` version of the method.)

```rust
    // Process the `get_response` into `ErgsBox`es which match our
    // `ergs_box_for_bounty_spec`
    let list_of_ergs_boxes =
        ErgsBox::process_explorer_response_custom(&get_response, ergs_box_for_bounty_spec).unwrap();

    // Acquire the first `ErgsBox` from the list
    let ergs_box_for_bounty = list_of_ergs_boxes[0].clone();

```

As can be seen above once the response was processed, a list of `ErgsBox`es which matched our spec were generated for us. We then cloned the first element of the list, as we only need to use a single `ErgsBox` for the bounty.

And lastly, we can move all of this logic into it's own function to keep things clean.

```rust
pub fn get_ergs_box_for_bounty(user_address: String, bounty_amount_in_nano_ergs: u64) -> ErgsBox {
    // Take the generalized `BoxSpec` from an `ErgsBox` and modify it
    // for our use case. Specifically change the address to be our
    // user's address, and change the value_range so that the box
    // has enough to cover the bounty amount.
    let ergs_box_for_bounty_spec = ErgsBox::box_spec()
        .modified_address(Some(user_address))
        .modified_value_range(Some(bounty_amount_in_nano_ergs..u64::MAX));
    // Acquire the Ergo Explorer API endpoint in order to find
    // the our `ergs_box_for_bounty`.
    let ergs_box_for_bounty_url = ergs_box_for_bounty_spec
        .explorer_endpoint("https://api.ergoplatform.com/api")
        .unwrap();
    // Make a get request to the Ergo Explorer API endpoint
    let get_response = get(&ergs_box_for_bounty_url).unwrap().text().unwrap();
    // Process the `get_response` into `ErgsBox`es which match our
    // `ergs_box_for_bounty_spec`
    let list_of_ergs_boxes =
        ErgsBox::process_explorer_response_custom(&get_response, ergs_box_for_bounty_spec).unwrap();

    // Return the first `ErgsBox` from the list
    list_of_ergs_boxes[0].clone()
}
```

Thus we only need to use the following line inside our `main` function in the `bounty` command section:

```rust
// Acquire the ergs_box_for_bounty
let ergs_box_for_bounty =
    get_ergs_box_for_bounty(user_address.clone(), bounty_amount_in_nano_ergs);
```

Now we can move forward to acquiring the `ergs_box_for_fee`. First let's define the function from the get-go this time:

```rust
pub fn get_ergs_box_for_fee(user_address: String, tx_fee: u64, ergs_box_for_bounty: ErgsBox) -> ErgsBox {

}
```

In this case we will take three inputs, the user's address, the transaction fee amount required, and the `ergs_box_for_bounty` we found previously. The reason we need the final input is because we have to make sure that the `ergs_box_for_fee` is not the same box as `ergs_box_for_bounty`, because we can only provide a single box once as an input within a transaction.

From here forward, the process is nearly identical to the previous function to acquire the `ErgsBox`. The code below is the result:

```rust
pub fn get_ergs_box_for_fee(
    user_address: String,
    tx_fee: u64,
    ergs_box_for_bounty: ErgsBox,
) -> ErgsBox {
    // Take the generalized `BoxSpec` from an `ErgsBox` and modify it
    // for our use case. Specifically change the address to be our
    // user's address, and change the value_range so that the box
    // has enough to cover the fee amount.
    let ergs_box_for_bounty_spec = ErgsBox::box_spec()
        .modified_address(Some(user_address))
        .modified_value_range(Some(tx_fee..u64::MAX));
    // Acquire the Ergo Explorer API endpoint in order to find
    // the our `ergs_box_for_bounty`.
    let ergs_box_for_bounty_url = ergs_box_for_bounty_spec
        .explorer_endpoint("https://api.ergoplatform.com/api")
        .unwrap();
    // Make a get request to the Ergo Explorer API endpoint
    let get_response = get(&ergs_box_for_bounty_url).unwrap().text().unwrap();
    // Process the `get_response` into `ErgsBox`es which match our
    // `ergs_box_for_bounty_spec`
    let list_of_ergs_boxes =
        ErgsBox::process_explorer_response_custom(&get_response, ergs_box_for_bounty_spec).unwrap();

    // If the two `ErgsBox`es are not equal, return the first box in the list
    if list_of_ergs_boxes[0] != ergs_box_for_bounty {
        return list_of_ergs_boxes[0].clone();
    } else {
        // Return the second `ErgsBox` from the list
        list_of_ergs_boxes[1].clone()
    }
}
```

As you may have noticed, there is one key difference. At the end of the function we check that the `ErgsBox` we are returning is not the same `ErgsBox` as our `ergs_box_for_bounty`. This is vital, otherwise the transaction created by the Action may be invalid due to using the same box twice. (Do note this logic requires the user to have at least 2 boxes in their wallet with sufficient number of Ergs inside. We have not performed any error checking in this tutorial series for edge cases such as this, in order to keep things easy to understand.)

And as before, we need to use the function inside of the `bounty` command section in our `main`:

```rust
// Acquire the ergs_box_for_fee
let ergs_box_for_fee =
    get_ergs_box_for_fee(user_address.clone(), tx_fee, ergs_box_for_bounty.clone());
```

### Creating And Issuing The "Bootstrap Math Bounty Box" Action Transaction

At this point in time, we have all the required inputs to use our `action_bootstrap_math_bounty_box` method that we implemented in our headless dApp. Doing so as below generates an `UnsignedTransaction` using all of the `ErgsBox`es we found and other inputs we acquired.

```rust
// Create the "Bootstrap Math Bounty Box" action unsigned
// transaction
let unsigned_tx = MathBountyProtocol::action_bootstrap_math_bounty_box(
    bounty_amount_in_nano_ergs,
    ergs_box_for_bounty,
    block_height,
    tx_fee,
    ergs_box_for_fee,
    user_address,
);
```

As we already have access to an unlocked ergo node/wallet from earlier in this tutorial, signing and submitting this `UnsignedTransaction` to the Ergo Blockchain is extremely trivial using the `ergo-node-interface` library.

```rust
// Sign and submit the transaction
let tx_id = node.sign_and_submit_transaction(&unsigned_tx).unwrap();

println!("Bootstrap Math Bounty Box Tx ID: {}", tx_id);
```

And just like that, we have finished implementing the front-end for the "Bootstrap Math Bounty Box" action from our headless dApp inside of a full-fledged CLI application.


Here is the final code from all of the above in the today's tutorial:

```rust
use ergo_node_interface::*;
use math_bounty_headless::*;
use reqwest::blocking::get;

fn main() {
    // Get a `NodeInterface`
    let node = acquire_node_interface_from_local_config();
    // Get the current Ergo Blockchain block height
    let block_height = node.current_block_height().unwrap();
    // Get the first address in the user's wallet
    let user_address = node.wallet_addresses().unwrap()[0].clone();

    // Acquire CLI arguments
    let args: Vec<String> = std::env::args().collect();
    let tx_fee = 1000000;

    if args.len() == 3 {
        // User wishes to submit Ergs to create a new `MathBountyBox`
        if args[1] == "bounty" {
            // Taking user input as Ergs and converting to nanoErgs
            let bounty_amount_in_nano_ergs = erg_to_nano_erg(args[2].parse::<f64>().unwrap());

            // Acquire the ergs_box_for_bounty
            let ergs_box_for_bounty =
                get_ergs_box_for_bounty(user_address.clone(), bounty_amount_in_nano_ergs);

            // Acquire the ergs_box_for_fee
            let ergs_box_for_fee =
                get_ergs_box_for_fee(user_address.clone(), tx_fee, ergs_box_for_bounty.clone());

            // Create the "Bootstrap Math Bounty Box" action unsigned
            // transaction
            let unsigned_tx = MathBountyProtocol::action_bootstrap_math_bounty_box(
                bounty_amount_in_nano_ergs,
                ergs_box_for_bounty,
                block_height,
                tx_fee,
                ergs_box_for_fee,
                user_address,
            );

            // Sign and submit the transaction
            let tx_id = node.sign_and_submit_transaction(&unsigned_tx).unwrap();

            println!("Bootstrap Math Bounty Box Tx ID: {}", tx_id);
        }
    }
}

pub fn get_ergs_box_for_bounty(user_address: String, bounty_amount_in_nano_ergs: u64) -> ErgsBox {
    // Take the generalized `BoxSpec` from an `ErgsBox` and modify it
    // for our use case. Specifically change the address to be our
    // user's address, and change the value_range so that the box
    // has enough to cover the bounty amount.
    let ergs_box_for_bounty_spec = ErgsBox::box_spec()
        .modified_address(Some(user_address))
        .modified_value_range(Some(bounty_amount_in_nano_ergs..u64::MAX));
    // Acquire the Ergo Explorer API endpoint in order to find
    // the our `ergs_box_for_bounty`.
    let ergs_box_for_bounty_url = ergs_box_for_bounty_spec
        .explorer_endpoint("https://api.ergoplatform.com/api")
        .unwrap();
    // Make a get request to the Ergo Explorer API endpoint
    let get_response = get(&ergs_box_for_bounty_url).unwrap().text().unwrap();
    // Process the `get_response` into `ErgsBox`es which match our
    // `ergs_box_for_bounty_spec`
    let list_of_ergs_boxes =
        ErgsBox::process_explorer_response_custom(&get_response, ergs_box_for_bounty_spec).unwrap();

    // Return the first `ErgsBox` from the list
    list_of_ergs_boxes[0].clone()
}

pub fn get_ergs_box_for_fee(
    user_address: String,
    tx_fee: u64,
    ergs_box_for_bounty: ErgsBox,
) -> ErgsBox {
    // Take the generalized `BoxSpec` from an `ErgsBox` and modify it
    // for our use case. Specifically change the address to be our
    // user's address, and change the value_range so that the box
    // has enough to cover the fee amount.
    let ergs_box_for_bounty_spec = ErgsBox::box_spec()
        .modified_address(Some(user_address))
        .modified_value_range(Some(tx_fee..u64::MAX));
    // Acquire the Ergo Explorer API endpoint in order to find
    // the our `ergs_box_for_bounty`.
    let ergs_box_for_bounty_url = ergs_box_for_bounty_spec
        .explorer_endpoint("https://api.ergoplatform.com/api")
        .unwrap();
    // Make a get request to the Ergo Explorer API endpoint
    let get_response = get(&ergs_box_for_bounty_url).unwrap().text().unwrap();
    // Process the `get_response` into `ErgsBox`es which match our
    // `ergs_box_for_bounty_spec`
    let list_of_ergs_boxes =
        ErgsBox::process_explorer_response_custom(&get_response, ergs_box_for_bounty_spec).unwrap();

    // If the two `ErgsBox`es are not equal, return the first box in the list
    if list_of_ergs_boxes[0] != ergs_box_for_bounty {
        return list_of_ergs_boxes[0].clone();
    } else {
        // Return the second `ErgsBox` from the list
        list_of_ergs_boxes[1].clone()
    }
}
```

### Testing And Running The Math Bounty CLI Application

Before we conclude, it would be be useful to explicitly explain how to test the application yourself with your own Ergo Node wallet.

Simply build the application via:

```rust
cargo build
```

Once built, your binary `math-bounty-cli` will be in the `target/debug` folder.

```sh
cd target/debug
```

After running your app the first time to generate the `node-interface.yaml` config file, you can test your application to ensure both the headless dApp and the frontend CLI both were implemented correctly by submitting 0.001 Ergs as a bounty.

```rust
./math-bounty-cli bounty 0.001
```

If successful, you will be told the transaction id of the "Bootstrap Math Bounty Box" action that was just submit to the Ergo Blockchain:

```rust
Bootstrap Math Bounty Box Tx ID: "c2c2287d642424ba9f7bb8757cde40cea540fd61c85ad830c46769a56c006ce2"
```


### Conclusion

With all of that said and done, you have finished creating your very first front-end for a headless dApp. The Ergo Headless dApp Framework provided a lot of helper methods/functions which made the experience quite streamlined. Furthermore, once you understand the basic patterns we touched upon in this tutorial, implementing all further actions for any headless dApp is effectively the same.

Some actions may require more/less user input, an extra GET request to some external API to fetch some other off-chain data, or a few more `SpecifiedBox` structs which you need to acquire via the explorer API. That said, the process of creating `UnsignedTransactions` by using these actions is just the same.

This provides significant power for frontend developers, because they only have to learn how to write frontends for headless dApps once, and they will have the power to build on top of each and every headless dApp publicly available, potentially creating new user experiences never thought possible before.

In the following tutorial we will be finishing our Math Bounty CLI application by implementing the textual front-end for our headless dApp's "Solve Math Problem" action. If you are feeling adventurous, I would highly recommend attempting to implement the front-end yourself before going through that tutorial when it releases, because all of the patterns you learned today will be reused to finish off the application.