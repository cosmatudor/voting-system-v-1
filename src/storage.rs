use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;
use multiversx_sc::api::StorageMapperApi;
use multiversx_sc::storage::StorageKey;

#[type_abi]
#[derive(Debug, NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, ManagedVecItem)]
pub struct Vote<M: ManagedTypeApi> {
    pub voter: ManagedAddress<M>,
    pub option: u64,
}

pub struct VotingCampaign<SA>
where
    SA: StorageMapperApi,
     {
    pub campaign_id: u64,
    pub owner: ManagedAddress<SA>,
    pub num_options: u64,
    pub votes: ManagedVec<SA, Vote<SA>>,
    pub voters:  MapMapper<SA, ManagedAddress<SA>, bool>,
}

impl<SA> VotingCampaign<SA>
where
    SA: StorageMapperApi,
{
    pub fn set(&mut self, campaign: VotingCampaign<SA>) {
        self.campaign_id = campaign.campaign_id;
        self.owner = campaign.owner;
        self.num_options = campaign.num_options;
        self.votes = campaign.votes;
        self.voters = campaign.voters;
    }
}

#[type_abi]
#[derive(Debug, NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, ManagedVecItem)]
pub struct VotingCampaignView<M: ManagedTypeApi> {
    pub campaign_id: u64,
    pub owner: ManagedAddress<M>,
    pub num_options: u64,
}

impl<SA> StorageMapper<SA> for VotingCampaign<SA>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        VotingCampaign {
            campaign_id: 0,
            owner: ManagedAddress::default(),
            num_options: 0,
            votes: ManagedVec::new(),
            voters: MapMapper::new(base_key),
        }
    }
}

impl<SA> StorageClearable for VotingCampaign<SA>
where
    SA: StorageMapperApi,
{
    fn clear(&mut self) {
        self.campaign_id = 0;
        self.owner = ManagedAddress::default();
        self.num_options = 0;
        self.votes.clear();
        self.voters.clear();
    }
}

#[multiversx_sc::module]
pub trait VotingStorage {
    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("num_campaigns")]
    fn num_campaigns(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("campaigns")]
    fn campaigns(&self, campaign_id: u64) -> VotingCampaign<Self::Api>;
}