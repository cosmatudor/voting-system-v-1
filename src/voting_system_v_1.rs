#![no_std]

// #[allow(unused_imports)]

use multiversx_sc::imports::*;
use storage::{Vote, VotingCampaignView};

mod storage;

#[multiversx_sc::contract]
pub trait VotingSystemV1: storage::VotingStorage {
    #[init]
    fn init(&self) {
        self.owner().set(self.blockchain().get_caller());
    }

    #[endpoint(createCampaign)]
    fn create_campaign(&self, num_options: u64) {
        let caller = self.blockchain().get_caller();
        let campaign_id = self.num_campaigns().get();

        let mut campaign = self.campaigns(campaign_id);
        campaign.campaign_id = campaign_id;
        campaign.owner = caller.clone();
        campaign.num_options = num_options;

        self.campaigns(campaign_id).set(campaign);

        self.num_campaigns().set(campaign_id + 1);

        self.create_transaction_event(campaign_id, caller, num_options);
    }

    // Allows a user to cast a vote for a given option.
    #[endpoint(vote)]
    fn vote(&self, campaign: u64, option: u64) {
        let mut campaign = self.campaigns(campaign);
        require!(
            option < campaign.num_options,
            "Invalid option!"
        );

        let caller = self.blockchain().get_caller();
        require!(
            !campaign.voters.contains_key(&caller),
            "You have already voted!"
        );
        // Mark the caller as having voted.
        campaign.voters.insert(caller.clone(), true);

        campaign.votes.push(Vote {
            voter: caller.clone(),
            option,
        });

        self.vote_event(campaign.campaign_id, caller, option);
    }

    // Returns all campaigns
    #[view(getAllCampaigns)]
    fn get_all_campaigns(&self) -> ManagedVec<VotingCampaignView<Self::Api>> {
        let num_campaigns = self.num_campaigns().get();
        let mut campaigns = ManagedVec::new();

        for i in 0..num_campaigns {
            let campaign = self.campaigns(i);
            campaigns.push(VotingCampaignView{
                campaign_id: campaign.campaign_id, 
                owner: campaign.owner, 
                num_options: campaign.num_options
            });
        }

        campaigns
    }

    // Returns all the votes for a given campaign.
    #[view(getVotesForCampaign)]
    fn get_total_options(&self, campaign_id: u64) -> ManagedVec<Vote<Self::Api>> {
        let campaign = self.campaigns(campaign_id);
        
        campaign.votes
    }

     // Returns the vote count for a given option.
     #[view(getVoteCount)]
     fn get_vote_count(&self, campaign_id: u64, option: u64) -> u64 {
         let campaign = self.campaigns(campaign_id);
         let votes = campaign.votes.iter().filter(|v| v.option == option).count() as u64;

         votes
     }


    /// Events

    #[event("createCampaignEvent")]
    fn create_transaction_event(
        &self,
        #[indexed] campaign_id: u64,
        #[indexed] owner: ManagedAddress,
        #[indexed] num_options: u64,
      );

    #[event("voteEvent")]
    fn vote_event(
        &self,
        #[indexed] campaign_id: u64,
        #[indexed] voter: ManagedAddress,
        #[indexed] option: u64,
    );
}