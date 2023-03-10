pub mod contract;
pub mod error;
pub mod msg;
mod staking;

pub const TENDERMINT_STAKING: &str = "abstract:tendermint-staking";

#[cfg(feature = "boot")]
pub mod boot {
    use abstract_boot::ApiDeployer;
    use abstract_os::api::InstantiateMsg;
    use boot_core::ContractWrapper;
    use boot_core::{boot_contract, BootEnvironment, Contract};
    use cosmwasm_std::Empty;

    use crate::msg::*;

    #[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
    pub struct TMintStakingApi<Chain>;

    impl<Chain: BootEnvironment> ApiDeployer<Chain, Empty> for TMintStakingApi<Chain> {}

    impl<Chain: BootEnvironment> TMintStakingApi<Chain> {
        pub fn new(name: &str, chain: Chain) -> Self {
            Self(
                Contract::new(name, chain)
                    .with_wasm_path("tendermint_staking")
                    .with_mock(Box::new(ContractWrapper::new_with_empty(
                        crate::contract::execute,
                        crate::contract::instantiate,
                        crate::contract::query,
                    ))),
            )
        }
    }
}
