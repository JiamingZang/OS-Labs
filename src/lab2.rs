use std::collections::VecDeque;
//按优先权调度算法实现处理器调度的程序
#[derive(Debug)]
pub enum Status {
    Ready,
    Working,
    Finish,
}

pub struct PCB{
    name: String,
    cpu_time:i32,
    priority:i32,
    estimated_running_time:i32,
    status:Status,

}

pub struct Priority{
    processes:VecDeque<PCB>,
}

impl PCB{


    pub fn print_process(&self){
        println!("{}     {}       {}        {:?}",self.name,self.estimated_running_time,self.priority,self.status);
    }

    pub fn execute(&mut self){
        match self.status{
            Status::Working=>{
                self.priority-=1;
                self.estimated_running_time-=1;
                self.cpu_time+=1;
                if self.estimated_running_time==0 {
                    self.status=Status::Finish
                }
            },
            Status::Ready=>{
                self.status = Status::Working;
                self.priority-=1;
                self.estimated_running_time-=1;
                self.cpu_time+=1;
                if self.estimated_running_time==0 {
                    self.status=Status::Finish
                }
            },
            _=>{},
        }
    }

    pub fn reset(&mut self) {
        match self.status {
            Status::Working=>{
                self.status=Status::Ready
            },
            _=>{},
        }
    }
}

impl Priority {

    
    
    pub fn initialize()->Priority{

        let mut processes = VecDeque::new();
        processes.push_back(PCB{name: "p1".to_string(),cpu_time:0,priority:1,estimated_running_time:2,status:Status::Ready});
        processes.push_back(PCB{name: "p2".to_string(),cpu_time:0,priority:5,estimated_running_time:3,status:Status::Ready});
        processes.push_back(PCB{name: "p3".to_string(),cpu_time:0,priority:3,estimated_running_time:1,status:Status::Ready});
        processes.push_back(PCB{name: "p4".to_string(),cpu_time:0,priority:4,estimated_running_time:2,status:Status::Ready});
        processes.push_back(PCB{name: "p5".to_string(),cpu_time:0,priority:2,estimated_running_time:4,status:Status::Ready});   

        Priority{processes: processes}
    }

    pub fn print_process(&self){
        println!("{} {} {} {}",format!("name"),format!("needtime"),format!("priority"),format!("state"));
        for p in self.processes.iter()  {
            p.print_process()
        }
        println!("________________")
    }

    pub fn reset(&mut self) {
        for p in self.processes.iter_mut() {
            p.reset()
        }
    }

    pub fn run_process(&mut self){
        let mut maxp=-10;
        let mut index=0;
        let mut count=0;
        self.reset();
        for p in self.processes.iter_mut() {
            
            if  p.priority>=maxp{
                match p.status{
                    Status::Finish =>{
                    },
                    _=>{
                        maxp = p.priority;
                        index = count;
                    }
                }
            }
            count += 1;
        }
        self.processes.get_mut(index).unwrap().execute();
        self.print_process();
    }

    pub fn is_finished(&mut self) -> bool {
        for p in self.processes.iter_mut() {
            match p.status {
                Status::Finish=>{},
                _ => return false,
            }
        }
        true
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_priority(){
        let mut priority = Priority::initialize();
        while !priority.is_finished(){
            priority.run_process();
        }
    }
}
