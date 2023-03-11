multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    PartialEq,
    Clone,
    Debug,
    TypeAbi,
)]
pub struct Space<M: ManagedTypeApi> {
    pub space_id: TokenIdentifier<M>,
    pub name: ManagedBuffer<M>,
}

#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    PartialEq,
    Clone,
    Debug,
    TypeAbi,
)]
pub struct Campaign<M: ManagedTypeApi> {
    pub space_id: TokenIdentifier<M>,
    pub nonce: u64,
    pub name: ManagedBuffer<M>,
    pub claim_amount: BigUint<M>,
    pub max_supply: BigUint<M>,
    pub minted_supply: BigUint<M>,
    pub start: u64,
    pub end: u64,
    pub created_date: u64,
    pub require_whitelist: bool,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("spaces")]
    fn spaces(&self) -> MapMapper<ManagedAddress, Space<Self::Api>>;

    #[view(getCampaigns)]
    #[storage_mapper("campaigns")]
    fn campaigns(&self, space_id: &TokenIdentifier) -> MapMapper<u64, Campaign<Self::Api>>;

    #[view(getCampaignWhiteList)]
    #[storage_mapper("campaign_whitelist")]
    fn campaign_whitelist(
        &self,
        space_id: &TokenIdentifier,
        nonce: u64,
    ) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getState)]
    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

    #[view(getSpaceState)]
    #[storage_mapper("space_is_paused")]
    fn space_is_paused(&self, space_id: &TokenIdentifier) -> SingleValueMapper<bool>;

    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;
}