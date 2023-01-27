use std::collections::VecDeque;

//模拟分页式存储管理中硬件的地址转换和产生缺页中断
pub struct Pagetable{
    pages:VecDeque<Pageinfo>,
    queue:VecDeque<usize>
}

pub struct Pageinfo {
    pagenum: usize,
    flag: bool,
    blocknum: usize,
    modifyflag: bool,
    location_on_disk:usize,
}

pub struct Instruction {
    operation:String,
    pagenum: usize,
    unitnum:usize,
}

impl Pagetable {

    pub fn initialize()->Pagetable {
        let mut pages = VecDeque::new();
        pages.push_back(Pageinfo{pagenum:0,flag:true,blocknum:5,location_on_disk:11, modifyflag: false });
        pages.push_back(Pageinfo{pagenum:1,flag:true,blocknum:8,location_on_disk:12, modifyflag: false});
        pages.push_back(Pageinfo{pagenum:2,flag:true,blocknum:9,location_on_disk:13, modifyflag: false});
        pages.push_back(Pageinfo{pagenum:3,flag:true,blocknum:1,location_on_disk:21, modifyflag: false});
        pages.push_back(Pageinfo{pagenum:4,flag:false,blocknum:0,location_on_disk:22, modifyflag: false});
        pages.push_back(Pageinfo{pagenum:5,flag:false,blocknum:0,location_on_disk:23, modifyflag: false});
        pages.push_back(Pageinfo{pagenum:6,flag:false,blocknum:0,location_on_disk:121, modifyflag: false});
        let mut queue = VecDeque::new();
        for p in pages.iter() {
            if p.flag {
                queue.push_back(p.pagenum);
            }
        };
        Pagetable {
            pages: pages,
            queue:queue,
        }
    }

    pub fn check_instruction(&mut self, instruction: &Instruction)->Result<String,usize> {
        for page in self.pages.iter_mut() {
            if page.flag && page.pagenum==instruction.pagenum{
                let res = 128*page.blocknum+instruction.unitnum;
                if instruction.operation=="存"{
                    page.modifyflag=true;
                }
                return Ok(res.to_string());
            }
        }
        return Err(instruction.pagenum)
    }

    pub fn fifo(&mut self,newpagenum:usize)->usize {
        let mut tempblocknum=0;
        let oldpagenum = self.queue.pop_front().unwrap();
        for p in self.pages.iter_mut() {
            if oldpagenum== p.pagenum{
                p.flag = false;
                tempblocknum = p.blocknum;
                p.blocknum=0;
            }
        }
        self.queue.push_back(newpagenum);
        for p in self.pages.iter_mut() {
            if p.pagenum==newpagenum{
                p.flag = true;
                p.modifyflag=false;
                p.blocknum = tempblocknum;
            }
        }
        oldpagenum
    }

    pub fn showpages(&mut self){
        println!("{}      {}    {}    {}","页号".to_string(),"标志".to_string(),"主存块号".to_string(),"位置".to_string());
        for page in self.pages.iter(){
            println!("{}         {}       {}        {}",page.pagenum,page.flag,page.blocknum,page.location_on_disk);
        }
    }

    pub fn check_modify(&mut self,pagenum:usize)-> bool{
        for p in self.pages.iter_mut() {
            if pagenum== p.pagenum{
                if p.modifyflag==true {
                    return true;
                }
            }
        }
        return false;
    }


}

impl Instruction {
    pub fn initialize()->VecDeque<Instruction> {
        let mut res = VecDeque::new();
        res.push_back(Instruction{operation:"+".to_string(), pagenum:0,unitnum:70});
        res.push_back(Instruction{operation:"+".to_string(), pagenum:1,unitnum:50});
        res.push_back(Instruction{operation:" ".to_string(), pagenum:2,unitnum:15});
        res.push_back(Instruction{operation:"存".to_string(), pagenum:3,unitnum:21});
        res.push_back(Instruction{operation:"取".to_string(), pagenum:0,unitnum:56});
        res.push_back(Instruction{operation:"-".to_string(), pagenum:6,unitnum:40});
        res.push_back(Instruction{operation:"移位".to_string(), pagenum:4,unitnum:53});
        res.push_back(Instruction{operation:"+".to_string(), pagenum:5,unitnum:23});
        res.push_back(Instruction{operation:"存".to_string(), pagenum:1,unitnum:37});
        res.push_back(Instruction{operation:"取".to_string(), pagenum:2,unitnum:78});
        res.push_back(Instruction{operation:"+".to_string(), pagenum:4,unitnum:1});
        res.push_back(Instruction{operation:"存".to_string(), pagenum:6,unitnum:84});
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_address_transformation() {
        let mut pagetable = Pagetable::initialize();
        let instructions = Instruction::initialize();
        println!("页号  单元号  绝对地址");
        for i in instructions.iter() {
            println!("{}      {}      {}",
                i.pagenum,
                i.unitnum, 
                match pagetable.check_instruction(i){
                    Ok(res) => format!("{}",res),
                    Err(err)=> format!("#{}",err),
                });
        }
    }

    #[test]
    pub fn test_fifo() {
        let mut pagetable = Pagetable::initialize();
        let instructions = Instruction::initialize();
        println!("页号  单元号   原页号  绝对地址");
        for i in instructions.iter() {
            println!("{}      {}      {}",
                i.pagenum,
                i.unitnum, 
                match pagetable.check_instruction(i){
                    Ok(res) => format!("         {}",res),
                    Err(err)=> {
                        let old = pagetable.fifo(err);

                        match  pagetable.check_instruction(i){
                            Ok(res)=> {

                                if pagetable.check_modify(old){
                                    format!("{}       {}，修改标志为true，写回磁盘",old,res.to_string())
                                }else{
                                    format!("{}       {}",old,res)
                                }
                            }
                            _=>format!("{}","err")
                        } 
                            
                    },
                });
        }
        pagetable.showpages();
    }
}
