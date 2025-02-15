use anyhow::Result;
use astria_core::sequencer::v1alpha1::{
    asset,
    Address,
};
use async_trait::async_trait;
use cnidarium::{
    StateRead,
    StateWrite,
};

#[async_trait]
pub(crate) trait ActionHandler {
    async fn check_stateless(&self) -> Result<()> {
        Ok(())
    }
    async fn check_stateful<S: StateRead + 'static>(
        &self,
        _state: &S,
        _from: Address,
        _fee_asset_id: asset::Id,
    ) -> Result<()> {
        Ok(())
    }
    async fn execute<S: StateWrite>(
        &self,
        _state: &mut S,
        _from: Address,
        _fee_asset_id: asset::Id,
    ) -> Result<()> {
        Ok(())
    }
}
