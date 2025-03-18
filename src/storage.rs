use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;
use multiversx_sc::api::StorageMapperApi;
use multiversx_sc::storage::StorageKey;

#[type_abi]
#[derive(Debug, NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, ManagedVecItem)]
pub struct VotingCampaign<SA>
where
    SA: StorageMapperApi,
     {
    pub campaign_id: u64,
    pub owner: ManagedAddress<SA>,
    pub public_key: ManagedBuffer<SA>,
    pub description: ManagedBuffer<SA>,
    pub num_options: u64,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub votes: ManagedVec<SA, ManagedBuffer<SA>>,
    // pub voters:  MapMapper<SA, ManagedAddress<SA>, bool>,
    pub is_tallied: bool,
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
        // self.voters = campaign.voters;
    }
}

#[type_abi]
#[derive(Debug, NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, ManagedVecItem)]
pub struct VotingCampaignView<M: ManagedTypeApi> {
    pub campaign_id: u64,
    pub owner: ManagedAddress<M>,
    pub description: ManagedBuffer<M>,
    pub num_options: u64,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub is_tallied: bool,
}

impl<SA> StorageMapper<SA> for VotingCampaign<SA>
where
    SA: StorageMapperApi,
{
    fn new(_: StorageKey<SA>) -> Self {
        VotingCampaign {
            campaign_id: 0,
            owner: ManagedAddress::default(),
            public_key: ManagedBuffer::default(),
            description: ManagedBuffer::default(),
            num_options: 0,
            start_timestamp: 0,
            end_timestamp: 0,
            votes: ManagedVec::new(),
            // voters: MapMapper::new(base_key),
            is_tallied: false,
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
        self.public_key = ManagedBuffer::default();
        self.description = ManagedBuffer::default();
        self.num_options = 0;
        self.start_timestamp = 0;
        self.end_timestamp = 0;
        self.votes.clear();
        // self.voters.clear();
        self.is_tallied = false;
    }
}

#[multiversx_sc::module]
pub trait VotingStorage {
    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("num_campaigns")]
    fn num_campaigns(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("campaigns")]
    fn campaigns(&self, campaign_id: u64) -> SingleValueMapper<VotingCampaign<Self::Api>>;
}