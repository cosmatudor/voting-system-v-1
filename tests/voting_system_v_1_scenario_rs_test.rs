use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/voting-system-v-1.mxsc.json", voting_system_v_1::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/voting-system-v-1.scen.json");
}
