use ammonia::Builder;
pub fn clean_full(s:String) -> String{
    return Builder::new().rm_tags(&["a","abbr","acronym","area","article","aside","b","bdi","bdo","blockquote","br","caption","center","cite","code",
    "col","colgroup","data","dd","del","details","dfn","div",
    "dl","dt","em","figcaption","figure","footer","h1","h2",
    "h3","h4","h5","h6","header","hgroup","hr","i","img",
    "ins","kbd","kbd","li","map","mark","nav","ol","p","pre",
    "q","rp","rt","rtc","ruby","s","samp","small","span",
    "strike","strong","sub","summary","sup","table","tbody",
    "td","th","thead","time","tr","tt","u","ul","var","wbr"]).clean(&s).to_string();
}