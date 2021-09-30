pub mod ip_tools{
    pub fn strip_port(ip:String)->String{
        let split = ip.split(":");
        let vec = split.collect::<Vec<&str>>();
        return vec[0].to_owned();
    }
}