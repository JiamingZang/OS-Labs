use std::collections::VecDeque;
//按时间片轮转法实现处理器调度的程序
#[derive(Debug)]
pub enum Status {
    R, 
    E,
}

pub struct PCB{
    name: String,
    estimated_running_time:i32,
    passed_time:i32,
    status:Status,

}

pub struct RR{
    processes: VecDeque<PCB>,
}


impl PCB{


    pub fn print_process(&self){
        println!("{}     {}       {}        {:?}",self.name,self.passed_time,self.estimated_running_time,self.status);
    }

    pub fn execute(&mut self){
        match self.status{
            Status::R=>{
                self.estimated_running_time-=1;
                self.passed_time+=1;
                if self.estimated_running_time==0 {
                    self.status=Status::E
                }
            },
            _=>{},
        }
    }

}

impl RR{
    pub fn initialize() -> RR {

        let mut processes = VecDeque::new();
        processes.push_back(PCB{name: "p1".to_string(),passed_time:0,estimated_running_time:2,status:Status::R});
        processes.push_back(PCB{name: "p2".to_string(),passed_time:0,estimated_running_time:3,status:Status::R});
        processes.push_back(PCB{name: "p3".to_string(),passed_time:0,estimated_running_time:1,status:Status::R});
        processes.push_back(PCB{name: "p4".to_string(),passed_time:0,estimated_running_time:2,status:Status::R});
        processes.push_back(PCB{name: "p5".to_string(),passed_time:0,estimated_running_time:4,status:Status::R});   

        RR{processes: processes}
    }


    pub fn print_process(&self){
        println!("{} {} {} {}",format!("name"),format!("passedtime"),format!("needtime"),format!("state"));
        for p in self.processes.iter()  {
            p.print_process()
        }
        println!("________________")
    }

    pub fn run_process(&mut self) {

        while !self.is_finished() {

            for p in self.processes.iter_mut() {
                match p.status{
                    Status::R=>{
                        p.execute();
                        println!("{} {} {} {}",format!("name"),format!("passedtime"),format!("needtime"),format!("state"));
                        p.print_process()
                    },
                    _=>{}
                }
            }
            // self.print_process();
            
            
        }
    }


    pub fn is_finished(&mut self) -> bool {
        for p in self.processes.iter_mut() {
            match p.status {
                Status::E=>{},
                _ => return false,
            }
        }
        self.print_process();
        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_rr() {
        let mut rr = RR::initialize();
        rr.run_process();
    }
}
