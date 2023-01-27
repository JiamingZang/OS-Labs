use std::collections::VecDeque;
//模拟文件管理的工作过程
pub struct MFDblock {
    username:String,
    files:VecDeque<UFDblock>,
    running_files:AFD,
}

pub struct UFDblock {
    filename:String,
    file_protect_code:Vec<bool>,
    open_protect_code:Vec<bool>,
    read:usize,
    write:usize,
    filelength:usize,
}

pub struct AFD {
    opened_files:VecDeque<String>,
}

pub struct Root{
    users:VecDeque<MFDblock>,
}

impl Root {

    pub fn new() -> Root {
        let mut users = VecDeque::new();
        let user1 = MFDblock::new("user1".to_string());
        let user2 = MFDblock::new("user2".to_string());
        users.push_back(user1);
        users.push_back(user2);
        Root{ users:users }
    }

    pub fn get_users(&mut self,username:String) ->Result<&mut MFDblock,String>{
        for u in self.users.iter_mut() {
            if u.username == username{
                return Ok(u)
            }
        }
        Err(format!("用户不存在"))
    }

}

impl MFDblock{
    
    pub fn new(username:String) -> MFDblock{
        MFDblock{
            username:username,
            files:VecDeque::new(),
            running_files:AFD::new(),
        }
    }

    pub fn add_ufd(&mut self,ufd:UFDblock){
        self.files.push_back(ufd);
    }
    
    pub fn display(&self){
        for block in self.files.iter(){
            println!("{} ", block.filename);
        }
    }

    pub fn create_file(&mut self){
        let mut filename = String::new();
        println!("请输入您要创建的文件名：");
        std::io::stdin().read_line(&mut filename).unwrap();
        self.files.push_back(UFDblock::new(filename.trim().to_string()));
    }

    pub fn delete_file(&mut self){
        let mut filename = String::new();
        let mut count = 0;
        println!("请输入您要删除的文件名：");
        std::io::stdin().read_line(&mut filename).unwrap();
        for file in self.files.iter_mut() {
            if file.filename == filename.trim().to_string() {
               break;
            }else {
                count+=1;
            }
        } 
        println!("{}",count);
        self.files.remove(count);
    }

    pub fn open_file(&mut self){
        let mut filename = String::new();
        println!("请输入您要打开的文件名：");
        std::io::stdin().read_line(&mut filename).unwrap();
        for file in self.files.iter_mut() {
            if file.filename == filename.trim().to_string(){
                if file.file_protect_code[0]==false&&file.open_protect_code[0]==false{
                    println!("无打开权限");
                }else if !self.running_files.is_file_opened(&filename){
                    self.running_files.add_opened_file(filename.clone());
                    println!("文件已经打开")
                }else{
                    println!("文件已经处于打开状态")
                }
            } 
        }
    }

    pub fn close_file(&mut self){
        let mut filename = String::new();
        println!("请输入您要关闭的文件名：");
        std::io::stdin().read_line(&mut filename).unwrap();
        for file in self.files.iter_mut() {
            if file.filename == filename.trim().to_string(){
                if self.running_files.is_file_opened(&filename){
                    self.running_files.remove_opened_file(filename.clone());
                    println!("文件已经关闭")
                }else{
                    println!("文件已经处于关闭状态")
                }
            } 
        }
    }

    pub fn read_file(&mut self){
        let mut filename = String::new();
        println!("请输入您要读出的文件名：");
        std::io::stdin().read_line(&mut filename).unwrap();
        for file in self.files.iter_mut() {
            if file.filename == filename.trim().to_string(){
                if file.file_protect_code[0]==false
                 ||file.open_protect_code[0]==false{
                     println!("无权限")
                }else if self.running_files.is_file_opened(&filename){
                    file.read+=1;
                    println!("文件已经读出")
                }else {
                    println!("文件还未开启")
                }
            } 
        }
    }

    pub fn write_file(&mut self){
        let mut filename = String::new();
        println!("请输入您要写入的文件名：");
        std::io::stdin().read_line(&mut filename).unwrap();
        for file in self.files.iter_mut() {
            if file.filename == filename.trim().to_string(){
                if file.file_protect_code[0]==false
                 ||file.open_protect_code[0]==false
                 ||file.file_protect_code[1]==false
                 ||file.open_protect_code[1]==false{
                     println!("无权限")
                }else if self.running_files.is_file_opened(&filename){
                    file.write+=1;
                    println!("文件已经写入")
                }else {
                    println!("文件还未开启")
                }
            } 
        }
    }

}

impl UFDblock {
    pub fn new(filename:String) -> UFDblock {
        let file_protect_code = vec![true,true,true];
        let open_protect_code = vec![true,true,true];
        UFDblock { 
            filename: filename,
            file_protect_code:file_protect_code,
            open_protect_code:open_protect_code,
            read:0,write:0,
            filelength:0,
        }
    }
}

impl AFD{
    pub fn new()->AFD{
        AFD{opened_files:VecDeque::new()}
    }

    pub fn add_opened_file(&mut self,filename:String){
        self.opened_files.push_back(filename)
    }

    pub fn remove_opened_file(&mut self,filename:String){
        let mut count = 0;
        for name in self.opened_files.iter() {
            if name == &filename {
                break;
            }else{
                count += 1;
            }
        }
        self.opened_files.remove(count);
    }

    pub fn is_file_opened(&self,filename:&String)->bool{
        for name in self.opened_files.iter() {
            if name == filename {
                return true
            }
        }
        false
    }
}

pub fn run(){
    let mut root = Root::new();
    let mut username = String::new();
    println!("请输入用户名：");
    std::io::stdin().read_line(&mut username).unwrap();
    match root.get_users(username.trim().to_string()) {
        Ok(u)=>{
            println!("欢迎：{}",u.username);
            let mut isrunning =true;
            while isrunning {

                let mut line = String::new();
                println!("请选择操作:");
                println!("1.显示所有文件 2.建立文件 3.删除文件 4.打开文件\n5.关闭文件 6.读出文件 7.写入文件 8.退出");
                std::io::stdin().read_line(&mut line).unwrap();
                match line.trim().parse::<u32>().unwrap() {
                    1=>{
                        u.display();
                    },
                    2=>{
                        u.create_file();
                    },
                    3=>{
                        u.delete_file();
                    },
                    4=>{
                        u.open_file();
                    },
                    5=>{
                        u.close_file();
                    },
                    6=>{
                        u.read_file();
                    },
                    7=>{
                        u.write_file();
                    },
                    8=>{
                        isrunning=false;
                    }
                    _=>{},
                }
            }
            
        },
        Err(err) => println!("{}",err)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_root_get_users() {
        let mut root = Root::new();
        let res= 
        match root.get_users("user1".to_string()) {
            Ok(u) => Ok(u),            
            Err(err) => {
                println!("{}", err);
                assert_eq!(err,format!("用户不存在"));
                Err(err)
            },
        };
    }

    #[test]
    pub fn test_root_run(){
        let file1 = UFDblock::new("file1".to_string());
        let file2 = UFDblock::new("file2".to_string());
        let mut user1 = MFDblock::new("user1".to_string());
        user1.add_ufd(file1);
        user1.add_ufd(file2);
        let mut root = Root::new();
        root.users.push_back(user1);
        // root.run();
    }

    #[test]
    pub fn test_MFD_display() {
        let file1 = UFDblock::new("file1".to_string());
        let file2 = UFDblock::new("file2".to_string());
        let mut user1 = MFDblock::new("user1".to_string());
        user1.add_ufd(file1);
        user1.add_ufd(file2);
        user1.display();
    }

    #[test]
    pub fn test_MFD_create_file(){
        let file1 = UFDblock::new("file1".to_string());
        let file2 = UFDblock::new("file2".to_string());
        let mut user1 = MFDblock::new("user1".to_string());
        user1.add_ufd(file1);
        user1.add_ufd(file2);
        user1.create_file();
        user1.display();
    }

    #[test]
    pub fn test_MFD_delete_file(){
        let file1 = UFDblock::new("file1".to_string());
        let file2 = UFDblock::new("file2".to_string());
        let mut user1 = MFDblock::new("user1".to_string());
        user1.add_ufd(file1);
        user1.add_ufd(file2);
        user1.delete_file();
        user1.display();
    }

    #[test]
    pub fn test_open_and_close_file(){
        let file1 = UFDblock::new("file1".to_string());
        let file2 = UFDblock::new("file2".to_string());
        let mut user1 = MFDblock::new("user1".to_string());
        user1.add_ufd(file1);
        user1.add_ufd(file2);
        user1.open_file();
        user1.close_file();
        user1.display();
    }

    #[test]
    pub fn test_read_file() {
        let file1 = UFDblock::new("file1".to_string());
        let file2 = UFDblock::new("file2".to_string());
        let mut user1 = MFDblock::new("user1".to_string());
        user1.add_ufd(file1);
        user1.add_ufd(file2);
        user1.read_file();
        user1.open_file();
        user1.read_file();
        user1.display();
    }

    #[test]
    pub fn test_write_file() {
        let file1 = UFDblock::new("file1".to_string());
        let file2 = UFDblock::new("file2".to_string());
        let mut user1 = MFDblock::new("user1".to_string());
        user1.add_ufd(file1);
        user1.add_ufd(file2);
        user1.write_file();
        user1.open_file();
        user1.write_file();
        user1.display();
    }

    #[test]
    pub fn test_run(){
        run();
    }
}
