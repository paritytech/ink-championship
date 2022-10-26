#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod player {
    use game::SquinkSplashRef;
    use ink::env::call::FromAccountId;

    #[ink(storage)]
    pub struct Player {
        game: SquinkSplashRef,
        dimensions: (u32, u32),
        seed: u32,
    }

    impl Player {
        #[ink(constructor)]
        pub fn new(game: AccountId) -> Self {
            let game = SquinkSplashRef::from_account_id(game);
            Self {
                dimensions: game.dimensions(),
                seed: Self::env().block_number(),
                game,
            }
        }

        /// This is the function that will be called during every game round.
        ///
        /// The function returns an `(x, y)` coordinate of the pixel which you
        /// want to color.
        ///
        /// # Notes
        ///
        /// The function signature `&mut self` is so that you can retain state
        /// in the contract's storage if you want to.
        ///
        /// The function can be named as you like, but it always needs to have
        /// a defined selector of `0`.
        #[ink(message, selector = 0)]
        pub fn your_turn(&mut self) -> (u32, u32) {
            let size = self.dimensions.0 * self.dimensions.1;
            for i in 0..size {
                let idx = (i + self.seed) % size;
                let turn = (idx % self.dimensions.0, idx / self.dimensions.0);
                if self.game.field(turn.0, turn.1).is_none() {
                    self.seed = idx;
                    return turn
                }
            }
            (0, 0)
        }
    }
}
