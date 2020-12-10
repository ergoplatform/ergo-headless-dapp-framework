# 2. Math Bounty dApp - Finishing The Off-Chain Library

In the first tutorial we went from 0 to having a functioning headless with a single action implemented. Granted, we only implemented half of our smart contract protocol (bootstrapping a Math Bounty box), and as such today we are going to fix this glaring hole by implementing the second half of our headless dApp.

## Recap

As you may recall from the previous tutorial, we created a `MathBountyBox` which implemented the `SpecifiedBox` trait. This means that our `MathBountyBox` has a `BoxSpec` attached to it as a method which defines exactly what kind of `ErgoBox` is a valid `MathBountyBox`. (This `BoxSpec` also automatically provides an extremely useful interface for front-end developers to find the required UTXOs for Actions on the Blockchain with next to 0 work on their end.)

Furthermore, thanks to the derive procedural macros `#[derive(WrapBox, SpecBox)]`, our `MathBountyBox` has several helper methods to make reading data from the box easier, as well as an auto-generated `new` method which automatically verifies that an `ErgoBox` matches our `BoxSpec` while creating a new `MathBountyBox`.

Thus we have an interface for both creating and using our `MathBountyBox`. This is the overarching design pattern which you will use for implementing both stages as well as more generic input boxes to Actions in your protocol.


## Using Your First Specified Box In An Action

As you may recalled, previously we used the `ErgsBox` struct for two of the inputs for our "BootStrap Math Bounty Box" Action. This `ErgsBox` struct is itself a `SpecifiedBox` which we got to take advantage of.

Today we are going to be writing our first Action using a `SpecifiedBox` that we created ourselves. That being our `MathBountyBox`.

The action which we will be implementing is the "Solve Math Problem" action. This action allows a user who knows the answer to the math problem we encoded inside of our smart contract to withdraw the bounty funds inside of a box at the Math Bounty stage (aka. a `MathBountyBox`).

As with the first action we wrote, we will be implementing this one on our `MathBountyProtocol` as a method:

```rust
impl MathBountyProtocol {
    pub fn action_solve_math_problem() -> UnsignedTransaction {
        todo!()
    }

    ...
}
```

This time the input arguments to our action method are going to be a bit different. We will need:
1. The answer to the math problem (which would be acquired from user-input in front-end)
2. The `MathBountyBox` which we will be spending in order to withdraw the bounty funds inside.
3. Current Block Height
4. Transaction Fee
5. ErgsBox For Fee
6. User Address


```rust
/// An action to solve the math problem inside of a `MathBountyBox`
/// and thus to withdraw the bounty nanoErgs inside as a reward.
pub fn action_solve_math_problem(
    math_problem_answer: u64,
    math_bounty_box: MathBountyBox,
    current_height: u64,
    transaction_fee: u64,
    ergs_box_for_fee: ErgsBox,
    user_address: String,
) -> UnsignedTransaction {
    todo!()
}
```

The majority of these arguments are the same, except this time around we are using the `math_problem_answer` in order to make the smart contract that the `math_bounty_box` is locked under evaluate to `true` and thus allowing us to spend it (withdraw the bounty funds).

To start off, let's fill out our method with the same boilerplate as from the first tutorial.

```rust
{
    let tx_inputs = vec![];
    let output_candidates = vec![];

    UnsignedTransaction::new(tx_inputs, vec![], output_candidates)
}
```

Just like in the previous action, we will not be using any data-inputs (the 2nd argument to `new`), and as such we can just fill that in as being empty from the start.

Next we will be filling in the `tx_inputs` vector. Because our smart contract did not have any checks on the inputs of the transaction, the order of our inputs makes no difference. Nonetheless, we will put the `math_bounty_box` first and the `ergs_box_for_fee` second, as this is usually the default that you will be using in more complex dApps in the future.

```rust
{
    let tx_inputs = vec![
        math_bounty_box.as_unsigned_input(),
        ergs_box_for_fee.as_unsigned_input(),
    ];
    let output_candidates = vec![];

    UnsignedTransaction::new(tx_inputs, vec![], output_candidates)
}
```

## Implementing The "Solve Math Problem" Action Logic

Now comes the fun part. We are going to be implementing the "Solve Math Problem" logic. As you may recall, this is the smart contract our headless dApp is using:

```scala
{
 OUTPUTS(0).R4[Long].get * 2 == 4
}
```

As such, inside of our "Solve Math Problem" action, we must create an `UnsignedTransaction` which:
- Has at least one output (Output 0)
- Output 0 has a Long integer inside of Register 4
- This Long integer inside of R4 will be the `math_problem_answer`

To move towards this, let's first calculate how many nanoErgs of bounty are left after we cover the transaction fee.

```rust
// Calculating the leftover bounty after paying for the tx fee
let bounty_after_fee = math_bounty_box.nano_ergs() - transaction_fee;
```

Now we can define the value of R4 of our output box.

```rust
// Converting our `math_problem_answer` from a `u64` to a `Constant`.
// This is the datatype that registers are encoded as inside of
// `ErgoBox`es. Note: register integers are signed, which is why
// we converted first to an `i64`, and then into a `Constant`.
let r4 = Constant::from(math_problem_answer as i64);
```

As mentioned in the above comment, registers inside of `ErgoBox`es are of the `Constant` datatype. Thus we must convert our `math_problem_answer` into a `Constant` using its `from` method.

With that out of the way, we can now create our output candidate which will fulfill the mathematical check encoded within our smart contract.

```rust
// An output candidate with the withdrawn bounty funds +  the answer to the
// math problem being held in R4.
let withdrawn_bounty_candidate = create_candidate(
    bounty_after_fee,
    &user_address,
    &vec![],
    &vec![r4],
    current_height,
).unwrap();
```

The above output candidate holds the resulting nanoErgs from the bounty after we have paid off the transaction fee, sent to the user's address, and holds the answer to the math problem in R4 as a Long integer (`i64` in rust as a `Constant`). This implements the logic for allowing user's to submit answers to the math problem according to the design of the smart contract.

Note: The list of registers as a part of `create_candidate` is an ordered list. This means that whatever `Constant` you put as the first element will be treated as R4, the second as R5, and so on up until R9.

With the majority of the logic implemented, all we have to do is create the transaction fee output candidate and insert the output candidates into the list in the correct order.

```rust
let transaction_fee_candidate =
    TxFeeBox::output_candidate(transaction_fee, current_height).unwrap();

let output_candidates = vec![withdrawn_bounty_candidate, transaction_fee_candidate];
```

Remember, because our smart contract checks for the `math_problem_answer` to be in R4 of Output 0, this mean that we **must** place our `withdrawn_bounty_candidate` as the first element in the `output_candidates` list. If we put it in a different spot in the list of outputs the transaction will fail even if the user provides the correct `math_problem_answer`.

With all of that said and done, we have now finished implementing our "Solve Math Problem" action, and as such, finished writing our pure & portable headless dApp. Here is the resulting code up to this point:


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
    /// An action to solve the math problem inside of a `MathBountyBox`
    /// and thus to withdraw the bounty nanoErgs inside as a reward.
    pub fn action_solve_math_problem(
        math_problem_answer: u64,
        math_bounty_box: MathBountyBox,
        current_height: u64,
        transaction_fee: u64,
        ergs_box_for_fee: ErgsBox,
        user_address: String,
    ) -> UnsignedTransaction {
        let tx_inputs = vec![
            math_bounty_box.as_unsigned_input(),
            ergs_box_for_fee.as_unsigned_input(),
        ];

        // Calculating the leftover bounty after paying for the tx fee
        let bounty_after_fee = math_bounty_box.nano_ergs() - transaction_fee;

        // Converting our `math_problem_answer` from a `u64` to a `Constant`.
        // This is the datatype that registers are encoded as inside of
        // `ErgoBox`es. Note: register integers are signed, which is why
        // we converted first to an `i64`, and then into a `Constant`.
        let r4 = Constant::from(math_problem_answer as i64);

        // A candidate with the withdrawn bounty funds +  the answer to the
        // math problem being held in R4.
        let withdrawn_bounty_candidate = create_candidate(
            bounty_after_fee,
            &user_address,
            &vec![],
            &vec![r4],
            current_height,
        )
        .unwrap();

        // Create the Transaction Fee box candidate
        let transaction_fee_candidate =
            TxFeeBox::output_candidate(transaction_fee, current_height).unwrap();

        let output_candidates = vec![withdrawn_bounty_candidate, transaction_fee_candidate];

        UnsignedTransaction::new(tx_inputs, vec![], output_candidates)
    }

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
As can be seen, writing your headless dApp using the Ergo Headless dApp Framework is actually not that complicated. There are indeed a few novel moving parts which you will have to learn and get use to using over time, but the benefits of doing so are palpable.

The Ergo Headless dApp Framework currently provides the best UTXO-based dApp development experience available on any blockchain to date, both from the core dApp developer's perspective, as well as a front-end implementors perspective. The reason for this is that thanks to the headless dApp design pattern, we have divided the smart contract protocol concerns from the front end implementation completely. This provides us with the advantage of having much more clean & portable code together with a brand new business model opened wide for all new dApps developed.

This opens up the horizon for dApp projects to allow anyone and everyone to build custom front-ends on top of their headless dApps. This creates a new income stream and business model for front-end developers, while encouraging enhanced decentralization of the entire ecosystem. This is a key novel opportunity that for the future of the entire dApp ecosystem on all blockchains.

Furthermore, as we will see in the next tutorial, using the HDF provides front-end implementors an extremely simplified interface for interacting with your headless dApp without having to understand the nitty-gritty details. This is a consequence of the way we've designed your library with as many protocol details as possible abstracted away from the front-end implementors. Thus they can just focus on what they are good at doing, developing front-ends that end-users enjoy using.

