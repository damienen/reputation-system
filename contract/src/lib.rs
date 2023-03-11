#![no_std]

use storage::{Campaign, Space};

multiversx_sc::imports!();

pub mod common_utils;
pub mod errors;
pub mod requirements;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait Reputation:
    storage::StorageModule
    + requirements::RequirementsModule
    + common_utils::CommonUtils
    + views::ViewsModule
{
    #[init]
    fn init(&self) {}

    #[endpoint(createSpace)]
    fn create_space(&self, token_identifier: TokenIdentifier, name: ManagedBuffer) {
        let caller = self.blockchain().get_caller();

        let space: Space<Self::Api> = Space {
            space_id: token_identifier,
            name,
        };

        self.spaces().insert(caller, space);
    }

    #[payable("*")]
    #[endpoint(createCampaign)]
    fn create_campaign(
        &self,
        name: ManagedBuffer,
        media: ManagedBuffer,
        claim_amount: BigUint,
        require_whitelist: bool,
        opt_supply: OptionalValue<BigUint>,
        opt_period: OptionalValue<MultiValue2<u64, u64>>,
    ) {
        self.require_is_ready();
        let caller = self.blockchain().get_caller();
        let space = self.get_space(&caller); // will panic if use doesn't have a space

        self.require_space_not_paused(&space.space_id); // requires space to be unpaused

        self.require_value_is_positive(&claim_amount);

        let (start, end) = opt_period
            .into_option()
            .unwrap_or_else(|| MultiValue2((0u64, 0u64)))
            .into_tuple();

        let max_supply = opt_supply.into_option().unwrap_or_default();

        if !require_whitelist {
            self.require_value_is_positive(&max_supply);
        }

        if start > 0u64 && end > 0u64 {
            self.require_time_period_is_valid(start, end);
        }

        let token_identifier = space.space_id.clone();

        if !require_whitelist {
            self.require_value_is_positive(&max_supply);
        }

        let nonce = self.send().esdt_nft_create(
            &space.space_id,
            &BigUint::from(1u64),
            &name,
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &00,
            &self.create_uris(media),
        );

        let created_date = self.blockchain().get_block_timestamp();

        let campaign: Campaign<Self::Api> = Campaign {
            space_id: space.space_id,
            nonce,
            name,
            claim_amount,
            max_supply,
            minted_supply: BigUint::zero(),
            start,
            end,
            created_date,
            require_whitelist,
        };

        self.campaigns(&token_identifier).insert(nonce, campaign);
    }

    #[only_owner]
    #[endpoint(pauseSpace)]
    fn pause_space(&self, space_id: TokenIdentifier) {
        self.space_is_paused(&space_id).set(true);
    }

    #[only_owner]
    #[endpoint(unpauseSpace)]
    fn unpause_space(&self, space_id: TokenIdentifier) {
        self.space_is_paused(&space_id).set(false);
    }

    #[only_owner]
    #[endpoint(pause)]
    fn pause(&self) {
        self.is_paused().set(true);
    }

    #[only_owner]
    #[endpoint(unpause)]
    fn unpause(&self) {
        self.is_paused().set(false);
    }

    #[only_owner]
    #[endpoint(setAdministrator)]
    fn set_administrator(&self, administrator: ManagedAddress) {
        self.administrator().set(&administrator);
    }
}
