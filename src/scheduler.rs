/* 
    Description:
        a scheduler, which decides the "what
        and when" of fuzzing. for a black box
        fuzzer like grampus, scheduling is not
        wide-spread to my knowledge. 
        however, I wanted to a have
        a scheduling unit in my fuzzer due to the 
        input regeneration. After a cycle with 
        `n` fuzzing iterations is complete, inputs
        are regenerated via the grammar. In between,
        I want to have control over what happens inside
        a mutation. Some mutations may be more expensive 
        than others, thus the scheduler can control how
        many mutations are done via a cycle in future
        implementations.

    Author: 0xca7
*/


/// different cycles the fuzzer can operate in
#[derive(Debug, Copy, Clone)]
pub enum FuzzingCycle {
    /// regenerate inputs
    CycleRegenerate,
    /// only bitflips, xors, arithmetic
    CycleDeterministic,
    /// remove chars, add chars + bitflips, xors, arithmetic
    CycleNonDeterministic,
}

pub struct Scheduler {
    /// current fuzzing cycle
    cycle:  FuzzingCycle,
    /// maximum iterations per cycle
    max_ips: usize,
    /// current iterations per cycle
    ips: usize,
}

impl Scheduler {

    /// instantiate a new Scheduler
    pub fn new(max_ips: usize) -> Scheduler {
        Scheduler {
            cycle: FuzzingCycle::CycleDeterministic,
            max_ips: max_ips,
            ips: 0,
        }
    }

    /// determine the new cycle, if a cycle switch
    /// occurs, set the boolean value returned from
    /// this function
    pub fn next(&mut self) -> (bool, FuzzingCycle) {

        if self.ips == self.max_ips {
            let next_cycle = match self.cycle {
                FuzzingCycle::CycleRegenerate => 
                    FuzzingCycle::CycleDeterministic,
                FuzzingCycle::CycleDeterministic => 
                    FuzzingCycle::CycleNonDeterministic,
                FuzzingCycle::CycleNonDeterministic =>
                    FuzzingCycle::CycleRegenerate,
            };

            self.cycle = next_cycle;
            self.ips = 0;

            return(true, next_cycle);
        } else {
            self.ips += 1;
        }

        // next cycle is the current cycle 
        (false, self.cycle)
    }

}


