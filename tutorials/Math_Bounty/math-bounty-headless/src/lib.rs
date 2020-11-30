pub use ergo_headless_dapp_framework::*;

#[derive(Debug, Clone, WrapBox, SpecBox)]
pub struct MathBountyBox {
    ergo_box: ErgoBox,
}

impl SpecifiedBox for MathBountyBox {
    fn box_spec() -> BoxSpec {
        let address = Some("94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string());
        BoxSpec::new(address, None, vec![], vec![])
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
