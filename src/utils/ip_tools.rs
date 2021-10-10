//TODO: optimize this for ipv6
//strip the port off of the ip request
pub fn strip_port(ip:String)->String{
    //split on ":"
    let split = ip.split(":");
    //collect into a vector
    let vec = split.collect::<Vec<&str>>();
    //return the stripped ip
    return vec[0].to_owned();
}
