//Copyright Venire Labs Inc 2019
//
// 
//

pub use super::read_only::{ReadOnlyOption, ReadState};

use super::{
    errors::{Error, result},
    INVALID_ID,
};

//Config begins a raft consensus

pub struct Config {

    //unique pid for the local raft
    pub id: u64,

    //peer private set upon init new raft
    pub peers: Vec<u64>,

    //learner node receive entries from leader node
    pub learners: Vec<u64>,

    //node.tick invocations that must pass between elections
    // election appointee based on when ElectionTick quorum took place
    // If a follower does not receive any message from the canditate
    // and start and election, he becomes a candidate
    pub election_tick: usize,


    //HeartbeatTick is the number of invocations to maintain current leadership
    pub heartbeat_tick: usize,

    pub applied: u64,

    pub max_size_per_msg: u64,

    pub max_inflight_msgs: usize,

    pub check_quorum: bool,

    pub pre_vote: bool,

    pub min_election_tick: usize,

    pub max_election_tick: usize,

    pub read_only_option: ReadOnlyOption,

    pub skip_bcast_commit: bool,

    //readable tag for logging
    pub tag: string,

}



impl Default for Config{
    fn default() -> Self {
        const HEARTBEAT_TICK: usize = 2;
            Self {
                id: 0,
                peers: vec![],
                learners: vec![],
                election_tick: HEARTBEAT_TICK * 10,
                heartbeat_tick: HEARTBEAT_TICK, 
                applied: 0,
                max_size_per_msg: 0,
                max_inflight_msgs: 256,
                check_quorum: false,
                pre_vote: false,
                min_election_tick: 0,
                max_election_tick: 0,
                read_only_option: ReadOnlyOption::Safe,
                skip_bcast_commit: false,
                tag: "".into(),
            }
    }
} 

impl Config {
    //Create a new config instance
    pub fn new(id:64 ) -> Self {
        Self {
            id,
            tag: format!("{}", id),
            ..Self::default()
        }

    }

    //The minimum number of ticks before an election
    #[inline]
    pub fn min_election_tick(&self) -> usize{
        if self.min_election_tick === 0 {
            self.election_tick
        } else {
            self.min_election_tick
        }
    }


    pub fn max_election_tick(&self) -> usize {
        if self.max_election_tick == 0 {

            2 * self.election_tick
        } else {
            self.max_election_tick
        }

    }

    //Run validation against config

}