/*
    Description:
        keep track of fuzzing stats
    
    Author:
        0xca7
*/
use std::time::Duration;

/// statistics for fuzzing
pub struct Stats {
    /// total fuzz cases
    total: u64,
    /// crashes encountered
    crashes: u64,
    /// cycles will be added in the future...
    cycles: u64,
}

impl Stats {

    pub fn new() -> Stats {
        Stats {
            total: 0,
            crashes: 0,
            cycles: 0,    
        }
    } // pub fn new

    pub fn inc_fuzz_cases(&mut self) {
        self.total += 1;
    }

    pub fn inc_cycles(&mut self) {
        self.cycles += 1;
    }
        
    pub fn inc_crashes(&mut self) {
        self.crashes += 1;
    }

    pub fn show_stats(&self, sec: &u64, elapsed: &Duration) {

        let fcps = self.total / sec;

        print!("\n[ Fuzzing Stats ]\n");
        print!("+----------------------------------+\n");
        print!("| [time]      {:?}\n", elapsed);
        print!("| [fcps]      {}\n", fcps);
        print!("| [total]     {}\n", self.total);
        print!("| [crashes]   {}\n", self.crashes);
        print!("| [cycles]    {}\n", self.cycles);
        print!("+----------------------------------+\n");
        // clear screen after each print.
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

}
