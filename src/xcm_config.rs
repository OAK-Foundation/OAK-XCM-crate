#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use cumulus_primitives_core::ParaId;
use frame_support::RuntimeDebug;
use sp_std::prelude::*;

#[derive(Encode, Decode, RuntimeDebug)]
pub enum AutomationTimeCall {
    #[codec(index = 2)]
    ScheduleXcmpTask(Vec<u8>, Vec<u64>, ParaId, Vec<u8>, u64),
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum NeuChainCall {
    #[codec(index = 60)]
    AutomationTime(AutomationTimeCall),
}

pub struct OakChainCallBuilder;

impl OakChainCallBuilder {
    pub fn automation_time_schedule_xcmp<T: frame_system::Config>(
        provided_id: Vec<u8>,
        execution_times: Vec<u64>,
        para_id: ParaId,
        returnable_call: Vec<u8>,
        weight_at_most: u64,
    ) -> NeuChainCall {
        NeuChainCall::AutomationTime(AutomationTimeCall::ScheduleXcmpTask(
            provided_id,
            execution_times,
            para_id,
            returnable_call,
            weight_at_most,
        ))
    }
}
