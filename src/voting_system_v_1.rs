#![no_std]

// #[allow(unused_imports)]

use fhe_traits::DeserializeParametrized;
use multiversx_sc::imports::*;
use fhe::{
    bfv::{Ciphertext}};
use storage::{VotingCampaign, VotingCampaignView};

mod storage;

#[multiversx_sc::contract]
pub trait VotingSystemV1: storage::VotingStorage {
    #[init]
    fn init(&self) {
        self.owner().set(self.blockchain().get_caller());
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(createCampaign)]
    fn create_campaign(
        &self, 
        description: ManagedBuffer,
        num_options: u64,
        start_timestamp: u64,
        end_timestamp: u64,
        public_key: ManagedBuffer,
    ) {
        require!(
            num_options >= 2,
            "Must have at least 2 options for voting"
        );
        require!(
            end_timestamp > start_timestamp,
            "End time must be after start time"
        );
        require!(
            !public_key.is_empty(),
            "Public key cannot be empty"
        );

        let caller = self.blockchain().get_caller();
        let campaign_id = self.num_campaigns().get();

        let campaign = VotingCampaign {
            campaign_id,
            owner: caller.clone(),
            public_key,
            description,
            num_options,
            start_timestamp,
            end_timestamp,
            votes: ManagedVec::new(),
            is_tallied: false,
        };

        self.campaigns(campaign_id).set(campaign);

        self.num_campaigns().set(campaign_id + 1);

        self.create_transaction_event(campaign_id, caller, num_options);
    }

    // Allows a user to cast a vote for a given option.
    #[endpoint(vote)]
    fn vote(&self, campaign_id: u64, vote: ManagedBuffer) {
        let mut campaign = self.campaigns(campaign_id).get();

        // Check if voting is active
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            current_timestamp >= campaign.start_timestamp,
            "Voting has not started yet"
        );
        require!(
            current_timestamp <= campaign.end_timestamp,
            "Voting has ended"
        );
        require!(
            !campaign.is_tallied,
            "Votes have already been tallied"
        );
        
        // Basic validation (empty check)
        require!(!vote.is_empty(), "Empty vote data");
        
        // Store the vote
        campaign.votes.push(vote);
    }

    fn tally_votes_for_campaign(&self, campaign_id: u64) {
        let mut campaign = self.campaigns(campaign_id).get();

        // Check if voting is active
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            current_timestamp >= campaign.end_timestamp,
            "Voting has not ended yet"
        );
        require!(
            !campaign.is_tallied,
            "Votes have already been tallied"
        );

        // Tally the votes
        for vote in campaign.votes.iter() {
            // let encrypted_vote = Ciphertext::from_bytes(vote, par); <--- This is where the decryption would happen, but par is of type std::sync::Arc; 
        }

        // Update the campaign
        campaign.is_tallied = true;
        campaign.votes.clear();

        self.campaigns(campaign_id).set(campaign);
    }

    // Returns all campaigns
    #[view(getAllCampaigns)]
    fn get_all_campaigns(&self) -> ManagedVec<VotingCampaignView<Self::Api>> {
        let num_campaigns = self.num_campaigns().get();
        let mut campaigns = ManagedVec::new();

        for i in 0..num_campaigns {
            let campaign = self.campaigns(i).get();
            campaigns.push(VotingCampaignView{
                campaign_id: campaign.campaign_id,
                owner: campaign.owner,
                description: campaign.description,
                num_options: campaign.num_options,
                start_timestamp: campaign.start_timestamp,
                end_timestamp: campaign.end_timestamp,
                is_tallied: campaign.is_tallied,
            });
        }

        campaigns
    }

    // Returns all the votes for a given campaign.
    #[view(getVotesForCampaign)]
    fn get_total_options(&self, campaign_id: u64) -> ManagedVec<ManagedBuffer> {
        let campaign = self.campaigns(campaign_id).get();
        
        campaign.votes
    }


    /// Events
    #[event("createCampaignEvent")]
    fn create_transaction_event(
        &self,
        #[indexed] campaign_id: u64,
        #[indexed] owner: ManagedAddress,
        #[indexed] num_options: u64,
      );
}