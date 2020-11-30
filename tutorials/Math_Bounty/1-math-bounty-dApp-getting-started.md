# 1. Math Bounty Headless dApp - Getting Started Writing Your First Action

In this tutorial series we will be building a simple "Math Bounty" headless dApp using the Ergo Headless dApp Framework. In short, this dApp allows individuals to lock Ergs up under a contract which requires a person to solve the math problem encoded in the contract in order to withdraw the funds inside. The idea for this dApp originally came from [this Ergo Forum Thread](https://www.ergoforum.org/t/mathematical-fun-with-ergoscript/76).

In our case we'll be using a simpler problem/contract to make it easy to follow along. Do note that this simplistic smart contract isn't intended for real world usage (because bad actors/bots can front-run your answer submission by watching the mempool). Nonetheless, this is an instructive example that you will be able to run live on testnet/mainnet for educational purposes. (Refer to the above linked thread for more details about how to make a more complicated, but secure Math Bounty smart contract)

In this first tutorial of the series, we will be covering the basics of how to get begin writing your headless dApp, from creating the project all the way to writing your first protocol action.

## The Smart Contract

Before we dive into building the headless dApp itself, let's take a look at the contract we'll be using.

```scala
{
 OUTPUTS(0).R4[Long].get * 2 == 4
}
```
As can be seen, the contract is extremely simple.

In short, a user can withdraw funds if they can figure out what number multiplied by `2` is equal to `4`. Specifically, funds can be withdrawn if the output UTXO with the index 0 has a register R4 that is a Long Integer is equal to the number `2` (in order for the equation to be true).

Compiling this contract into a P2S address results in the address: `94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr`
([Click here to try compiling it for yourself on the PlutoMonkey Playground](https://wallet.plutomonkey.com/p2s/?source=ewogSU5QVVRTKDApLlI0W0xvbmddLmdldCAqIDIgPT0gNAp9))


In the rest of this tutorial we will begin writing the headless dApp which performs all of the transaction creation logic.


## Preparing Your Project

We are going to be coding in Rust so ensure that you have the tooling installed on your machine: [Install Rust](https://www.rust-lang.org/tools/install)

Once installed you will have access to the `cargo` command in your terminal. We are going to be creating a new project called `math-bounty-headless`.

```rust
cargo new math-bounty-headless --lib
```

Cargo will create a new project folder for you called `math-bounty-headless`. Within the newly created `Cargo.toml` file inside of the project folder we will need to add a couple dependencies to get started using the HDF. In the `[dependencies]` section add:

```rust
ergo-headless-dapp-framework      = "0.1.0"
ergo-lib                          = "0.4.0"
```

You may have noticed that we included `ergo-lib` as a dependency as well. This is the go-to rust library for all of the core types/structs/functionality. The HDF exposes everything we'll need in our current project, but if your dApp gets sufficiently advanced you may eventually need to use the `ergo-lib` directly yourself.

Now we can jump over to the `src/lib.rs` file and get started coding.


## Writing And Specifying Your First Stage

First we're going to import all of the Ergo-related types and the Ergo Headless dApp Framework structs/functions/macros for use in our project:

```rust
pub use ergo_headless_dapp_framework::*;
```

At this point we will begin crafting the components of headless dApp by starting off with the stages of our protocol. In our case we have a simple single-stage smart contract protocol. This means we need to only create a single Rust stage-representing struct for our headless dApp.

This stage in will be called the `Math Bounty` stage. As such, we will name the struct which will wrap an `ErgoBox` at this stage the `MathBountyBox`.

```rust
pub struct MathBountyBox {
    ergo_box: ErgoBox,
}
```

Now that we've defined the `MathBountyBox`, we can also derive a few traits and helper methods automatically:

```rust
#[derive(Debug, Clone, WrapBox, SpecBox)]
pub struct MathBountyBox {
    ergo_box: ErgoBox,
}
```

To the Rust-initiated, `Debug` and `Clone` are typical, but `WrapBox` and `SpecBox` are novel. These are procedural macros which automatically implements the `WrappedBox` trait and a `new` method tied to the `SpecifiedBox` trait for our `MathBountyBox`. In other words, we have access to new helper methods without writing any extra code ourselves thanks to these macros. (Note: You must import `HeadlessDappError` if you ever derive `SpecBox`. We do this automatically by importing * in this project.)

Next we are going to implement the `SpecifiedBox` trait on our `MathBountyBox`, in order to take advantage of the `SpecBox` derive.

```rust
impl SpecifiedBox for MathBountyBox {
    fn box_spec() -> BoxSpec {
        todo!();
    }
}
```

Now this is where things get interesting. This trait requires us to implement a method which returns a `BoxSpec`.

A `BoxSpec` is a specification in the form of a Rust struct which specifies parameters of an `ErgoBox`. This spec struct is used as a "source of truth" to both verify and find `ErgoBox`es which match the spec.

As such, we are going to create a `BoxSpec` for the Math Bounty stage, which will be used by our `MathBountyBox` struct. We will be doing this using the `BoxSpec::new()` function which allows us to specify the address, value range, registers, and tokens for our specification. In our case we will only be using the address due to the simplicity of our smart contract.


```rust
impl SpecifiedBox for MathBountyBox {
    fn box_spec() -> BoxSpec {
        let address = Some("94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string());
        BoxSpec::new(address, None, vec![], vec![])
    }
}
```

Our Rust-based spec of the Math Bounty stage states that an `ErgoBox` which has an address of `94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr` is a valid `MathBountyBox`. Furthermore this means that no matter what Ergs/registers/tokens the `ErgoBox` has inside, it is still considered a valid `MathBountyBox`. For our simple use case, this is a valid spec for what are going for.

Lastly to finish up with the `MathBountyBox`, we are going to implement the `new` method so that a `MathBountyBox` can be created.

```rust
impl MathBountyBox {
    pub fn new(ergo_box: &ErgoBox) -> Option<MathBountyBox> {
        // Using the automatically implemented `verify_box` method
        // from the `BoxSpec` to verify the `ErgoBox` is a valid
        // `MathBountyBox`.
        Self::box_spec().verify_box(ergo_box).ok()?;

        // Creating the `MathBountyBox`
        let math_bounty_box = MathBountyBox {
            ergo_box: ergo_box.clone(),
        };

        // Returning the `MathBountyBox`
        Some(math_bounty_box)
    }
}
```
As can be seen above, we use the `verify_box` method to ensure that the `ErgoBox` is indeed a valid `MathBountyBox` according to the spec we defined. This `verify_box` method is automatically available for use once the `SpecifiedBox` trait has been implemented, as we did above.

## Defining The Smart Contract Protocol

Going forward, we are going to begin defining the actions of our smart contract protocol for our headless dApp. Before we get there, first we must create an empty struct which represents our protocol. In our case, we are just going to call it `MathBountyProtocol`.


```rust
pub struct MathBountyProtocol {}
```

With a struct that represents our smart contract protocol we can implement protocol actions as methods on said struct. Thus as we expose this `MathBountyProtocol` publicly it will be easy for front-ends to be implemented on top of our headless dApp.

We will now begin to write our first action, `Bootstrap Math Bounty Box`, by making it a method.

```rust
impl MathBountyProtocol {
    /// A bootstrap action which allows a user to create a `MathBountyBox`
    /// with funds locked inside as a bounty for solving the math problem.
    pub action_bootstrap_math_bounty_box() -> UnsignedTransaction (
        todo!()
    }
}
```

When writing actions with the Ergo Headless dApp Framework, we must keep in mind that we are building pure, portable, and reusable code.

What this means is that all of our transaction creation logic within our actions must be self-contained. This is why we are creating/returning an `UnsignedTransaction` from our action. Furthermore this means that any external data (from the blockchain, or user input) must be provided to the action method via arguments. Thus for our `Bootstrap Math Bounty Box` action, we will need the following inputs:

```rust
    pub fn action_bootstrap_math_bounty_box(
        user_address: String,
        bounty_amount_in_nano_ergs: u64,
        ergs_box_for_bounty: ErgsBox,
        current_height: u64,
        transaction_fee: u64,
        ergs_box_for_fee: ErgsBox,
    ) -> UnsignedTransaction
```

The current height is required for tx building, the transaction fee is to be decided by the front-end implementor when the action method is used, the `ergs_box_for_fee` is a wrapped `ErgoBox` which is used to pay for the fee for the transaction, and the user's address is required to send change back to the user. These are the minimum arguments required for the majority of actions you will ever write.

Furthermore, in our current scenario, we also have the `ergs_box_for_bounty` and `bounty_amount_in_nano_ergs` input arguments. In the front-end the user will provide the amount of nanoErgs they want to submit as a bounty, and the front-end implementation for our headless dApp must find an input `ErgsBox` with sufficient nanoErgs to cover the bounty amount which is owned by the user.

This is actually a lot simpler than it all may sound thanks to the HDF implementing a number of key helper methods on top of `SpecifiedBox`s (an `ErgsBox` being one of the already implemented `SpecifiedBox`es by the HDF) for acquiring UTXOs easily. This will all be tackled in a future tutorial once we are working on building a front-end for our headless dApp.

Next let's write the basic scaffolding for creating our `UnsignedTransaction` that we are returning in our method:

```rust
{
    let tx_inputs = vec![];
    let data_inputs = vec![];
    let output_candidates = vec![];

    UnsignedTransaction::new(tx_inputs, data_inputs, output_candidates)
}
```

As can be seen, to create an unsigned transaction we simply need three things:
1. An ordered list of input boxes
2. An ordered list of data-inputs
3. An ordered list of output box candidates

In our case we will not be using any data-inputs (aka. read-only inputs) because our protocol is simple and does not rely on reading any other UTXOs on the blockchain. Thus we will move forward by defining our `tx_inputs` and our `output_candidates` in such a way that the resulting `UnsignedTransaction` is a valid implementation of our `Bootstrap Math Bounty Box` action.

Let's implement the inputs first as these are very simple.

```rust
let tx_inputs = vec![
    ergs_box_for_bounty.as_unsigned_input(),
    ergs_box_for_fee.as_unsigned_input(),
];
```

All we are doing here is making the bounty `ErgsBox` the first input (index 0) and the fee `ErgsBox` the second input (index 1). To convert from a `SpecifiedBox`(or `WrappedBox`, which `ErgsBox`es are both) into an input which we can use with `UnsignedTransaction::new`, we simply call the `.as_unsigned_input()` method.

Thus we've already completed 2/3 of the requirements for creating an `UnsignedTransaction`, but now we get to the more exciting part where we encode the logic of our action.


### Implementing Action Logic

At this point we need to create the following output box candidates for our action:
1. The Math Bounty Box Candidate
2. Transaction Fee Box Candidate
3. The Change Box Candidate

Once we have all of these built, we can feed them to `UnsignedTransaction::new`, and officially finish implementing our action.

To get started towards this goal we first need to figure out how much extra change is held within the `ErgsBox`es that were provided as inputs. This is very simple math:

```rust
let total_nano_ergs = ergs_box_for_bounty.nano_ergs() + ergs_box_for_fee.nano_ergs();
let total_change = total_nano_ergs - bounty_amount_in_nano_ergs - transaction_fee;
```

In short, whatever we don't use for the bounty or the tx fee has to go back to the user as change.

Now with that out of the way we can begin creating our output candidates. First we are going to create our Math Bounty Box output candidate. This will be via the `create_candidate` function provided by the HDF.

```rust
// Creating our Math Bounty Box output candidate
let math_bounty_candidate = create_candidate(
    bounty_amount_in_nano_ergs,
    &"94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string(),
    &vec![],
    &vec![],
    current_height,
)
.unwrap();
```

The `create_candidate` function takes the following inputs:
1. nanoErgs to be held in the resulting output box.
2. The address which the output box will be at.
3. Tokens that the output box will hold.
4. Register values that the box will hold.
5. Current block height

In our case we simply want to create an output at the P2S smart contract address we compiled at the start of this tutorial, `94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr`, and for it to hold an amount of nanoErgs equal to `bounty_amount_in_nano_ergs`. We do not require any tokens to be held in the output, nor any data in the registers, and as such we leave these empty.

As a result, we have created our Math Bounty Box output candidate which will lock the bounty Ergs under the smart contract. Now we can finish off building the two final candidates for our action.

### Creating The Tx Fee And Change Boxes

Rather than manually using the `create_candidate` function for every single output candidate we are building, which can get tedious, the HDF provides us with some default "output builders". These are structs that offer associated functions which build output candidates more easily.

Thus to create a tx fee box output candidate, all we have to do is:
```rust
let transaction_fee_candidate =
    TxFeeBox::output_candidate(transaction_fee, current_height).unwrap();
```

Similarly the HDF also offers an output builder for a change box which we will use as well:

```rust
let change_box_candidate = ChangeBox::output_candidate(
    &vec![],
    total_change,
    &user_address,
    current_height,
)
```

And just like that we've finished creating all three required output candidates for our action. To reiterate, the three output box candidates we created were for the:
1. The Math Bounty Box
2. Transaction Fee Box
3. The Change Box

With the output box candidates complete all we have left to do is add the candidates into our list of `output_candidates` in the correct order. (In our case, our smart contract only specifies that the Math Bounty Box must be the first output)

```rust
let output_candidates = vec![
    math_bounty_candidate,
    transaction_fee_candidate,
    change_box_candidate,
];
```

And with that we have implemented the `Bootstrap Math Bounty Box` action for our headless dApp's smart contract protocol.

This is the final code from everything we've accomplished in this tutorial:

```rust
use ergo_headless_dapp_framework::*;

#[derive(Debug, Clone, WrapBox)]
pub struct MathBountyBox {
    ergo_box: ErgoBox,
}

impl SpecifiedBox for MathBountyBox {
    fn box_spec() -> BoxSpec {
        let address = Some("94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string());
        BoxSpec::new(address, None, vec![], vec![])
    }
}

impl MathBountyBox {
    pub fn new(ergo_box: &ErgoBox) -> Option<MathBountyBox> {
        // Using the automatically implemented `verify_box` method
        // from the `BoxSpec` to verify the `ErgoBox` is a valid
        // `MathBountyBox`.
        Self::box_spec().verify_box(ergo_box).ok()?;

        // Creating the `MathBountyBox`
        let math_bounty_box = MathBountyBox {
            ergo_box: ergo_box.clone(),
        };

        // Returning the `MathBountyBox`
        Some(math_bounty_box)
    }
}

pub struct MathBountyProtocol {}

impl MathBountyProtocol {
    /// A bootstrap action which allows a user to create a `MathBountyBox`
    /// with funds locked inside as a bounty for solving the math problem.
    pub fn action_bootstrap_math_bounty_box(
        bounty_amount_in_nano_ergs: u64,
        ergs_box_for_bounty: ErgsBox,
        current_height: u64,
        transaction_fee: u64,
        ergs_box_for_fee: ErgsBox,
        user_address: String,
    ) -> UnsignedTransaction {
        let tx_inputs = vec![
            ergs_box_for_bounty.as_unsigned_input(),
            ergs_box_for_fee.as_unsigned_input(),
        ];

        // Calculating left over change nanoErgs
        let total_nano_ergs = ergs_box_for_bounty.nano_ergs() + ergs_box_for_fee.nano_ergs();
        let total_change = total_nano_ergs - bounty_amount_in_nano_ergs - transaction_fee;

        // Creating our Math Bounty Box output candidate
        let math_bounty_candidate = create_candidate(
            bounty_amount_in_nano_ergs,
            &"94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string(),
            &vec![],
            &vec![],
            current_height,
        )
        .unwrap();

        // Create the Transaction Fee box candidate
        let transaction_fee_candidate =
            TxFeeBox::output_candidate(transaction_fee, current_height).unwrap();

        // Create the Change box candidate
        let change_box_candidate =
            ChangeBox::output_candidate(&vec![], total_change, &user_address, current_height)
                .unwrap();

        // Our output candidates list, specifically with the Math Bounty box
        // candidate being the first, meaning Output #0.
        let output_candidates = vec![
            math_bounty_candidate,
            transaction_fee_candidate,
            change_box_candidate,
        ];

        UnsignedTransaction::new(tx_inputs, vec![], output_candidates)
    }
}
```


## Conclusion

Congratulations, you've finished the first tutorial and currently have a working headless Math Bounty dApp!

At this point in time you have all the logic implemented to create a real `UnsignedTransaction` which you can submit to an Ergo node in order to lock a bounty under the Math Bounty smart contract. This headless dApp can be used programmatically in scripts/bots, or have a front-end built out which accepts user input. It is truly self-contained in such that it contains all the logic required to perform the actions of the smart contract protocol headlessly.

In the following tutorials we will be working on finishing this dApp by adding:
1. Support for the "Solve Math Problem" action to finish off our headless dApp.
2. Building a CLI front-end to out headless dApp.
3. Adding WASM support to make this headless dApp truly portable.

If you have any questions/comments/ideas, feel free to drop by the [Ergo Discord](https://discord.gg/kj7s7nb) and chat with the rest of the community.
