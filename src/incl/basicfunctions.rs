use crate::*;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct Nstring {
    // Nscript String stuff
}
impl NeoCatStructBinding for Nstring{
    fn neocat_exec(&mut self,tocall:&str,args: &Vec<NeoCatVar>) -> NeoCatVar {
        match tocall{
            "replace" => {
                if args.len() > 2{
                    let value = Nstring::replace(&args[0].stringdata, &args[1].stringdata, &args[2].stringdata);
                    let mut neovar = NeoCatVar::new("result","string");
                    neovar.stringdata = value.to_string();
                    return neovar;
                }else{
                    print("string::replace arguments missing, returing nothing","r");
                }
            }
            "between" => {
                if args.len() > 2{
                    let value = Nstring::stringbetween(&args[0].stringdata, &args[1].stringdata, &args[2].stringdata);
                    let mut neovar = NeoCatVar::new("result","string");
                    neovar.stringdata = value.to_string();
                    return neovar;
                }else{
                    print("string::between arguments missing, returing nothing","r");
                }
            }
            "contains" =>{
                if args.len() > 1{
                    let value = Nstring::instring(&args[0].stringdata, &args[1].stringdata);
                    let mut neovar = NeoCatVar::new("result","string");
                    neovar.stringdata = value.to_string();
                    return neovar;
                }else{
                    print("string::contains arguments missing, returing nothing","r");
                }
            }
            "split" =>{
                if args.len() > 0{
                    let mut tosplit = "";
                    if args.len() > 1 {
                        tosplit = &args[1].stringdata;
                    }
                    let mut neovar = NeoCatVar::new("result","array");
                    for xitem in split(&args[0].stringdata,&tosplit){
                        neovar.stringvec.push(xitem.to_string());
                    }

                    return neovar;
                }else{
                    print("string::split arguments missing, returing nothing","r");
                }
            }
            "join" =>{
                if args.len() > 0{
                    let mut tojoin = "";
                    if args.len() > 1 {
                        tojoin = &args[1].stringdata;
                    }
                    let mut neovar = NeoCatVar::new("result","string");
                    neovar.stringdata = args[0].stringvec.join(&tojoin);
                    return neovar;
                }else{
                    print("string::split arguments missing, returing nothing","r");
                }
            }
            "fromleft" =>{
                if args.len() > 1{
                    let mut neovar = NeoCatVar::new("result","array");
                    let string = Nstring::fromleft(&args[0].stringdata, args[1].stringdata.parse::<usize>().unwrap_or(0));
                    neovar.stringdata = string;
                    return neovar;
                }else{
                    print("string::fromleft arguments missing, returing nothing","r");
                }
            }
            "fromright" =>{
                if args.len() > 1{
                    let mut neovar = NeoCatVar::new("result","array");
                    let string = Nstring::fromright(&args[0].stringdata, args[1].stringdata.parse::<usize>().unwrap_or(0));
                    neovar.stringdata = string;
                    return neovar;
                }else{
                    print("string::fromright arguments missing, returing nothing","r");
                }
            }
            "trimright" =>{
                if args.len() > 1{
                    let mut neovar = NeoCatVar::new("result","array");
                    let string = Nstring::trimright(&args[0].stringdata, args[1].stringdata.parse::<usize>().unwrap_or(0));
                    neovar.stringdata = string;
                    return neovar;
                }else{
                    print("string::trimright arguments missing, returing nothing","r");
                }
            }
            "trimleft" =>{
                if args.len() > 1{
                    let mut neovar = NeoCatVar::new("result","array");
                    let string = Nstring::trimleft(&args[0].stringdata, args[1].stringdata.parse::<usize>().unwrap_or(0));
                    neovar.stringdata = string;
                    return neovar;
                }else{
                    print("string::trimleft arguments missing, returing nothing","r");
                }
            }
            _ =>{
                println!("cant find the [{}] function to call",&tocall);
            }
        }
        return NeoCatVar::new("nothing","string");
    }
}
impl Nstring {

    pub fn replace(s: &str, f: &str, r: &str) -> String {
        if f == "" || s == ""{
            //println!("debugger cannot replace none?{} by none?{} ",&s,&f);
            return s.to_string();
        }
        // i know slaat nergens op.. :P
        return s.replace(f, r);
    }

    pub fn instring(s: &str, f: &str) -> bool {
        let  r: bool;
        match s.find(f) {
            Some(_) => r = true,
            None => r = false,
        }
        return r;
    }
    pub fn trimleft(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len+1 {
            return String::from(&s[f..len]);
        }
        else {

            return s.to_string();
        }
        //return String::from(&s[f..len]);
    }
    pub fn trimright(s: &str, f: usize) -> String {
        let len = s.len();
        if s.len() > f {
            return String::from(&s[0..len - f]);
        }
        else {

            return s.to_string();
        }

    }
    pub fn fromleft(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[0..f]);
        } else {
            return String::new();
        }
    }
    pub fn fromright(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[len - f..len]);
        } else {
            return String::new();
        }
    }
    pub fn stringtoeval(s: &str) -> String {
        // saver for hashmap keys usages
        let mut r = s.replace("-", "_");
        let all = [
            "~", "!", "#", "%", "^", "&", "*", "(", ")", "\\", "{", "}", "[", "]", ".", ",", "?",
            "'", "$", "/",
        ];
        for c in all {
            r = r.replace(c, "_");
        }
        r
    }
    /// returns the value between 2 search strings
    pub fn stringbetween<'a>(str: &'a str, a: &str, b: &str) -> String {
        if let Some(start_pos) = str.find(a) {
            let rest = &str[start_pos + a.len()..];
            if let Some(end_pos) = rest.find(b) {
                let extracted = &rest[..end_pos];
                //return extracted.trim_start_matches(|c: char| c.is_whitespace()).trim_end_matches(|c: char| c.is_whitespace()).to_string();

                return extracted.to_string();
            }
        }
        "".to_owned()
    }
    pub fn stringbetweenincludeempty<'a>(str: &'a str, a: &str, b: &str) -> String {
        // used for interal usage to extraxt scopes, if a scope is empty its still a scope.
        // iteratrs shoulnd exit then so this funtion retuns something else
        // to let the iterator know to continue instead of a empty string.
        // ---------------------------------------
        if let Some(start_pos) = str.find(a) {
        let rest = &str[start_pos + a.len()..];
        if let Some(end_pos) = rest.find(b) {
            let extracted = &rest[..end_pos];
            //return extracted.trim_start_matches(|c: char| c.is_whitespace()).trim_end_matches(|c: char| c.is_whitespace()).to_string();

                return extracted.to_string();
        }
    }
    "<nonefound!>".to_owned()
    }
    pub fn tohexplus(s:&str)->String{//,["er","₹"]
        let mut hexplus = string_to_hex(s);
        let torep = [
            [",","G"],[".","H"],["=","I"],["/","J"],["-","K"],["_","L"],["*","M"],["(","N"],[")","O"],["{","P"],["[","Q"],["]","R"],["<","S"],[">","T"],["!","U"],["@","V"],["#","W"],["$","X"],["%","Y"],["^","Z"],

            ["ing","!"],
            ["\nt",""],["\ni","Ι"],["\na","Λ"],["\no","Μ"],
            ["the","∑"],["and","∫"],["for","∆"],["liv","È"],["spe","É"],
            [" th","™"],[" he","©"],[" in","●"],[" an","■"],["es ","★"],[" re","Â"],[" cl","¶"],[" te","·"],[" sp","µ"],[" fi","³"],[" di","²"],[" wh","Ώ"],
            ["er ","♦"],["th ","€"],["e ","~"],["t ","+"],["g ","|"],["s ","\\"],
            [" t",";"],[" a",":"],[" s","["],[" c","]"],[" o","_"],[" b","<"],[" y",">"],[" r","Ÿ"],[" w","¡"],[" h","¦"],[" l","§"],[" m","À"],
            ["ch","¥"],["ll","£"],["ea","¢"],["ou","."],["ma",","],["th","@"],["he","#"],["in","%"],["an","&"],["es","*"],["on","("],["re","$"],["wh","Œ"],["cc","Ž"],["mb","š"],["ss","ƒ"],
            ["rt","…"],["pp","†"],["gr","‡"],["pr","‰"],["pa","¹"],["ho","º"],["iv","»"],["sp","¼"],["xt","½"],["mp","¾"],["se","¿"],["ay","Á"],["qu","Ͷ"],["ri","ͷ"],["gh","ͻ"],["by","ͼ"],["nt","ͽ"],["nc","Ϳ"],["ca","Ͱ"],["op","ͱ"],["cl","ͳ"],
            ["he","Ͳ"],["nv","Ή"],["rs","Ί"],["ti","Ό"],["ex","Ύ"],["nf","ΐ"],["hi","Α"],["nd","Ζ"],["mm","Η"], ["rm","Θ"],["to","Β"],["ta","Γ"],["yo","Ν"],["ry","Ξ"],["oo","Ο"],["nn","Π"],["gg","Ρ"],["sh","Σ"],

            ["a","z"],["b","y"],["c","x"],["d","w"],["e","v"],["f","h"],["i","k"],["k","l"],["p","o"],["\n","`"]
        ];
        for rep in torep{
            hexplus = Nstring::replace(&hexplus, &string_to_hex(rep[0]), rep[1]);
        }
        return hexplus;
    }
    pub fn fromhexplus(s:&str)->String{
        let mut hexplus = s.to_string();
        let torep = [
            [",","G"],[".","H"],["=","I"],["/","J"],["-","K"],["_","L"],["*","M"],["(","N"],[")","O"],["{","P"],["[","Q"],["]","R"],["<","S"],[">","T"],["!","U"],["@","V"],["#","W"],["$","X"],["%","Y"],["^","Z"],

            ["ing","!"],
            ["\nt",""],["\ni","Ι"],["\na","Λ"],["\no","Μ"],
            ["the","∑"],["and","∫"],["for","∆"],["liv","È"],["spe","É"],
            [" th","™"],[" he","©"],[" in","●"],[" an","■"],["es ","★"],[" re","Â"],[" cl","¶"],[" te","·"],[" sp","µ"],[" fi","³"],[" di","²"],[" wh","Ώ"],
            ["er ","♦"],["th ","€"],["e ","~"],["t ","+"],["g ","|"],["s ","\\"],
            [" t",";"],[" a",":"],[" s","["],[" c","]"],[" o","_"],[" b","<"],[" y",">"],[" r","Ÿ"],[" w","¡"],[" h","¦"],[" l","§"],[" m","À"],
            ["ch","¥"],["ll","£"],["ea","¢"],["ou","."],["ma",","],["th","@"],["he","#"],["in","%"],["an","&"],["es","*"],["on","("],["re","$"],["wh","Œ"],["cc","Ž"],["mb","š"],["ss","ƒ"],
            ["rt","…"],["pp","†"],["gr","‡"],["pr","‰"],["pa","¹"],["ho","º"],["iv","»"],["sp","¼"],["xt","½"],["mp","¾"],["se","¿"],["ay","Á"],["qu","Ͷ"],["ri","ͷ"],["gh","ͻ"],["by","ͼ"],["nt","ͽ"],["nc","Ϳ"],["ca","Ͱ"],["op","ͱ"],["cl","ͳ"],
            ["he","Ͳ"],["nv","Ή"],["rs","Ί"],["ti","Ό"],["ex","Ύ"],["nf","ΐ"],["hi","Α"],["nd","Ζ"],["mm","Η"], ["rm","Θ"],["to","Β"],["ta","Γ"],["yo","Ν"],["ry","Ξ"],["oo","Ο"],["nn","Π"],["gg","Ρ"],["sh","Σ"],

            ["a","z"],["b","y"],["c","x"],["d","w"],["e","v"],["f","h"],["i","k"],["k","l"],["p","o"],["\n","`"]
        ];
        for rep in torep{
            hexplus = Nstring::replace(&hexplus, rep[1],&string_to_hex(rep[0]));
        }
        return hex_to_string(&hexplus);

    }
}

pub struct Ntimer {

}
impl NeoCatStructBinding for Ntimer{
    fn neocat_exec(&mut self,tocall:&str,args: &Vec<NeoCatVar>) -> NeoCatVar {
        let mut thisvar = NeoCatVar::new("error","string");
        match tocall{
            "init" => {
                let var = Ntimer::init();
                thisvar.stringdata = var.to_string();
            }
            "diff" => {
                if args[0].stringdata != ""{
                    let var = Ntimer::diff(args[0].stringdata.parse::<i64>().unwrap_or(0));
                    thisvar.stringdata = var.to_string();
                }
            }
            _ =>{
                println!("unknown function called from struct timer {}",tocall);
            }
        }
        return thisvar;
    }
}
impl Ntimer {
    pub fn init() -> i64 {
        // sets a timestamp in a i64 (in nscript_fn_bindings converted to strings)
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        return duration.as_millis() as i64;
    }

    pub fn diff(timerhandle: i64) -> i64 {
        // given a timestamp from init() it will give the timedifference in MS
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        return duration.as_millis() as i64 - timerhandle;
    }
    pub fn hours_in_ms(time: &str) -> String {
        return "".to_owned() + &(f64(&time)* f64(&"3600000")).to_string() ;
    }
    pub fn minutes_in_ms(time: &str) -> String {
        return "".to_owned() + &(f64(&time)* f64(&"60000")).to_string() ;
    }
    pub fn days_in_ms(time: &str) -> String {
        return "".to_owned() + &(f64(&time)* f64(&"86400000")).to_string() ;
    }
    pub fn weeks_in_ms(time: &str) -> String {
        return "".to_owned() + &(f64(&time)* f64(&"604800000")).to_string() ;
    }
    pub fn months_in_ms(time: &str) -> String {
        return "".to_owned() + &(f64(&time)* f64(&"2629800000")).to_string() ;
    }
    pub fn years_in_ms(time: &str) -> String {
        return "".to_owned() + &(f64(&time)* f64(&"31557600000")).to_string() ;
    }
}
fn f64(string: &str) -> f64{
    return string.parse::<f64>().unwrap_or(0.0);
}

pub fn hex_to_string(hex_string: &str) -> String {
    match Vec::from_hex(hex_string) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}

pub fn string_to_hex(input: &str) -> String {
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let bytes = input.as_bytes();
    let mut hex_string = String::new();

    for byte in bytes {
        let high_nibble = (byte & 0xF0) >> 4;
        let low_nibble = byte & 0x0F;
        hex_string.push(hex_chars[high_nibble as usize]);
        hex_string.push(hex_chars[low_nibble as usize]);
    }

    hex_string
}
pub fn string_to_eval(string_: &str) -> String {
    let mut return_val = string_.to_string();

    let replacements = [
        ("#", ""), ("%", ""), ("-", "_"), (" ", "_"), (":", "_"), ("\\", "_"), ("/", "_"),
        (".", "_"), ("@", "_"), ("&", "_"), ("!", ""), ("'", ""), ("[", "_"), ("]", "_"),
        ("(", "_"), (",", "_"), ("^", "_"), (")", "_"), ("|", "_")
    ];

    for (search, replace) in replacements {
        return_val = return_val.replace(search, replace);
    }

    return return_val;
}

pub fn print(m: &str, color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
    match color {
        "bright blue" | "bb" => {
            println!("{}", m.bright_blue());
        }
        "bright green" | "bg"=> {
            println!("{}", m.bright_green());
        }
        "bright cyan" | "bc" => {
            println!("{}", m.bright_cyan());
        }
        "bright red" | "br" => {
            println!("{}", m.bright_red());
        }
        "bright magenta" | "bm" => {
            println!("{}", m.bright_magenta());
        }
        "bright yellow" | "by" => {
            println!("{}", m.bright_yellow());
        }
        "bright purple" | "bp" => {
            println!("{}", m.bright_purple());
        }
        "purple" | "p" => {
            println!("{}", m.purple());
        }
        "cyan" | "c" =>{
            println!("{}", m.cyan());
        }
        "yellow" | "y" => {
            println!("{}", m.yellow());
        }
        "red" | "r" => {
            println!("{}", m.red());
        }
        "green" | "g" => {
            println!("{}", m.green());
        }
        "blue" | "b" =>{
            println!("{}", m.blue());
        }
        "magenta" | "m" =>{
            println!("{}", m.magenta());
        }

        _ => {
            println!("{}", m);

        }
    };
}

