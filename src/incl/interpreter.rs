

use crate::*;
pub type NeoCatSimpleFunctions = fn(Vec<NeoCatVar>) -> Option<NeoCatVar>;
pub fn emptyfnbuffer(_vec: Vec<NeoCatVar>) -> Option<NeoCatVar> {
    // Default behavior
    None
}
/// NeoCatScript main struct
pub struct NeoCat<'a>{
    // for user created structs
    pub ruststructs: HashMap<&'a str, &'a mut dyn NeoCatStructBinding>, // map for all the rust fn bindings.
    pub rustfunctions: HashMap<String, NeoCatSimpleFunctions>, // map for all the rust fn bindings.
    pub globalvars: HashMap<String,NeoCatVar>,
    pub classes: HashMap<String,NeoCatClass>,// all classes
    pub functions: HashMap<String,NeoCatFunc>,// all nonclass functions
    //pub threads: HashMap<String,NeoCatThread>,// all nonclass functions
    pub coroutines: Vec<String>,// all nonclass functions
    pub codeblocks: HashMap<String,NeoCatCodeBlock>,// all nonclass functions
    pub emptyblock: NeoCatCodeBlock,// all nonclass functions
    pub threadsreceiver: HashMap<String, mpsc::Receiver<NeoCatVar>>,
    pub threadssenders: HashMap<String, mpsc::Sender<NeoCatVar>>,
    //pub mainthread: HashMap<String, mpsc::Sender<String>>,
}

impl <'a> NeoCat<'a>{
    pub fn new() -> NeoCat<'a> {
        let mut this = NeoCat {
            ruststructs: HashMap::new(),
            rustfunctions: HashMap::new(),
            globalvars: HashMap::new(),
            classes: HashMap::new(),
            functions: HashMap::new(),
            //threads: HashMap::new(),
            coroutines: Vec::new(),
            codeblocks: HashMap::new(),
            emptyblock: NeoCatCodeBlock::new("emptyblock"),
            threadsreceiver:HashMap::new(),
            threadssenders:HashMap::new(),
            //mainthread:HashMap::new(),
        };
        //this.test();
        this.setcmdarguments();
        //this.parsefile("./test.nc");
        this
    }
    /// inserts a Rust function into the fnmap users can create their own function bindings using
    /// this. functions are required to have the NeoCatFunc Trait implemented.
    fn insertstruct(&mut self, key: &'a str, value: &'a mut dyn  NeoCatStructBinding)   {
        self.ruststructs.insert(key, value);
    }
    pub fn setcmdarguments(&mut self){
        let args: Vec<String> = env::args().collect();
        let mut i = 0;
        for givenarg in args.clone() {

            //println!("env:{}",&givenarg);
            let v = "".to_owned() + &givenarg.to_owned();
            let key ="$cmdarg".to_owned() + &i.to_string();
            //vmap.setvar(key, &v);
            let mut argvar = NeoCatVar::new(&key,"string");
            argvar.stringdata = v.to_string();
            self.setglobal(&key, argvar);
            i +=1;
        }
    }
    pub fn threadsend(&mut self,threadname:&str,vartosend:NeoCatVar) ->NeoCatVar{
        let tothread = "thread_".to_string() + &threadname;
             match self.threadssenders.get(&tothread){
                Some(sender) => {
                match sender.send(vartosend){
                    Ok(_)=>{
                            //println!("main send succes!");
                            match self.threadsreceiver.get(&tothread){
                                Some(receiver) =>{
                                   let msg: NeoCatVar = match receiver.try_recv(){
                                        Ok(m) =>m,
                                        Err(_) =>NeoCatVar::new("error","string"),
                                    };
                                    match msg.stringdata.as_str(){
                                        _ =>{
                                            if msg.stringdata.as_str() != ""{
                                                //println!("main sent{} received:{}",&tothread,msg.stringdata);
                                                return msg;
                                            }
                                        }
                                    }
                                },
                                None => {
                                    println!("no thread [{}] receiver channel found!",&tothread);
                                }
                            }
                        },
                    Err(_)=>{
                            //println!("main[{}] send error! msg({})",&param1,&param2);
                            return NeoCatVar::new("error","string");
                    }
                };
                    return NeoCatVar::new("ok","string");
                }
                None => {
                    println!("no threads found");
                    return NeoCatVar::new("ok","string");

                }
            };
    }
    pub fn removecoroutine(&mut self,routine:&str){
        self.coroutines.retain(|x| x != routine);
    }
    pub fn addcoroutine(&mut self,routine:&str){
        let string = routine.to_string();
        if self.coroutines.contains(&string) != true {
            self.coroutines.push(string);
        }
    }
    ///used internally to perform binded rust fn calls during interpretation
    // fn getfn(&mut self, key: &str) -> Option<&'a mut dyn NeoCatFnBinding> {
    //    self.fnmap.get(key)
    // }
    pub fn getglobal(&mut self,name:&str) ->NeoCatVar{
        if let Some(res) = self.globalvars.get_mut(name){
            return res.clone();
        }
        NeoCatVar::new(name,"string")//<-not found,ret new
    }
    pub fn setglobal(&mut self,name:&str,data:NeoCatVar){
        self.globalvars.insert(name.to_string(), data);
    }
    pub fn insertclass(&mut self,name:&str,class:NeoCatClass){
        self.classes.insert(name.trim().to_string(),class);
    }
    pub fn getclass(&mut self,name:&str)->NeoCatClass{
        if let Some(thisclass) = self.classes.get_mut(name){
            thisclass.copyto(name)
        }
        else{
            NeoCatClass::new("")
        }
    }
    pub fn getclassref(&mut self,name:&str)->Option<&mut NeoCatClass>{
        if let Some(thisclass) = self.classes.get_mut(name.trim()){
            Some(thisclass)
        }
        else{
            None
        }
    }
    pub fn getblock(&mut self,blockref:&str)->NeoCatCodeBlock{

        if let Some(this) = self.codeblocks.get_mut(blockref){
            //print(&format!("returning a block for [{}]",&blockref),"g");
            return this.copy();
        }
        else{
            //print(&format!("returning a emptyblock for [{}]",&blockref),"r");
            return self.emptyblock.copy();
        }
    }
    pub fn getblockref(&mut self,blockref:&str)->Option<&mut NeoCatCodeBlock>{

        if let Some(this) = self.codeblocks.get_mut(blockref){
            //print(&format!("returning a block for [{}]",&blockref),"g");
            return Some(this);
        }
        else{
            //print(&format!("returning a emptyblock for [{}]",&blockref),"r");
            return None;
        }
    }
    // pub fn getvar(&mut self,name:&str)->NeoCatVar{
    //
    // }

}
// pub struct NeoCatThread{
//     receiver: mpsc::Receiver<NeoCatVar>,
//     sender: mpsc::Sender<NeoCatVar>,
//     codestring: String,
// }
// impl NeoCatThread{
//     pub fn new() -> NeoCatThread{
//     let (main_to_worker_tx, main_to_worker_rx) = mpsc::channel();
//     let (worker_to_main_tx, worker_to_main_rx) = mpsc::channel();
//         NeoCatThread{
//             receiver: worker_to_main_rx,
//             sender: main_to_worker_tx,
//             codestring: "".to_string(),
//         }
//     }
// }
/// this struct contains the vectors with code
pub struct NeoCatCodeBlock{
    pub name: String,
    pub codeblock: String,// used for tokenizing
    pub codeblockvector: Vec<Vec<String>>,// formatted (all scopes extracted leftover code , init)
    pub subblockmap: Vec<Vec<Vec<String>>>,// all the subscopes, if else loop coroutines
    pub insubblock: usize,// all the subscopes, if else loop coroutines
    pub variables: HashMap<String,NeoCatVar>,// scope variables
    pub variableindex: Vec<String>,// scope variables
    pub staticstrings: Vec<String>,// scope variables
    pub ifscopedepth: usize,//used for parsing nested ifscopes
    pub ifscopes: Vec<bool>,// used for nested elseif else scopes
    pub inloop: usize, // used for nested loops
    pub breakloop: Vec<bool>, // used to break the right nested loop.
}

impl NeoCatCodeBlock{
    pub fn new(nameref:&str) -> NeoCatCodeBlock{
        let mut this = NeoCatCodeBlock{
            name: nameref.to_string(),
            codeblock: String::new(),
            codeblockvector: Vec::new(),
            subblockmap: Vec::new(),
            insubblock: 0,
            variables: HashMap::new(),
            variableindex: Vec::new(),
            staticstrings: Vec::new(),
            ifscopedepth: 0,
            ifscopes: Vec::new(),
            inloop: 0,
            breakloop: Vec::new(),
        };
        this.ifscopes.push(false);
        this.breakloop.push(false);
        this
    }
    pub fn copy(&mut self) ->NeoCatCodeBlock{
        let mut this = NeoCatCodeBlock{
            name: self.name.to_string(),
            codeblock: self.codeblock.clone(),
            codeblockvector: self.codeblockvector.clone(),
            subblockmap: self.subblockmap.clone(),
            insubblock: 0,
            variables: HashMap::new(),
            variableindex: self.variableindex.clone(),
            staticstrings: self.staticstrings.clone(),
            ifscopedepth: 0,
            ifscopes: Vec::new(),
            inloop: 0,
            breakloop: Vec::new(),

        };
        for xstatic in self.staticstrings.clone(){
            let thisvar = self.getvar(&xstatic);
            this.variables.insert(xstatic.to_string(),thisvar);
        }
        for xvar in self.variableindex.clone(){
            let thisvar = self.getvar(&xvar);
            this.variables.insert(xvar.to_string(),thisvar);
        }
        this.breakloop.push(false);
        this.ifscopes.push(false);
        this.variables.insert("self".to_string(),self.getvar("self"));
        this
    }

    pub fn subblocktostring(&mut self,subblock:usize) -> String{
        let mut outputstring = "".to_string();
        for xline in self.subblockmap[subblock-1].clone(){
            outputstring = outputstring + &xline.join(" ") + "\n";
        }
        return outputstring
    }
    pub fn ifset(&mut self,set:bool){
        self.ifscopes[self.ifscopedepth] = set;
    }
    pub fn ifdown(&mut self){
        self.ifscopedepth -=1;
        self.ifscopes =self.ifscopes[0..self.ifscopes.len()-1].to_vec();
    }
    pub fn ifup(&mut self){
        self.ifscopes.push(false);
        self.ifscopedepth +=1;
    }

    /// pre-formatting: all the converted hexed static strings will be assinged a variable
    fn convertstaticstrings(&mut self){
        let mut parsingtext = self.codeblock.to_string();
        let mut staticstringcount = 0;
        let chars = ["\n"," ",",",")","]"];
        for xchar in chars{
            loop {
                let mut hexstring = Nstring::stringbetween(&parsingtext, "^", &xchar);
                if hexstring == ""{ // break loop when block is done
                    break;
                }
                for xchar2 in chars{// get first on the line.
                    hexstring = split(&hexstring,&xchar2)[0].to_string();
                }
                if hexstring != "" {
                    let stringvarname = "~".to_string() + &staticstringcount.to_string();

                    staticstringcount +=1;
                    for xchar2 in chars{
                        let torep = "^".to_string() + &hexstring + &xchar2;
                        let repwith = stringvarname.to_string() + &xchar2;
                        parsingtext = Nstring::replace(&parsingtext, &torep, &repwith);
                        //print(&format!("{} rep: [{}] with: [{}] <- {}",&xchar,&torep,&repwith,&parsingtext),"r");
                    }
                    //let mut newvar = NeoCatVar::new(&stringvarname,"string");
                    let stringdata = hex_to_string(&hexstring);
                    //newvar.stringdata = stringdata;
                    //self.variables.insert(stringvarname.to_string(),newvar);
                    self.staticstrings.push(stringdata.to_string());
                }
            }
        }
        self.codeblock = parsingtext.to_string();
    }
    /// tokenizing and assinging subscopes
    pub fn formatblock(&mut self) {
        self.subblockmap = Vec::new();
        self.convertstaticstrings();
        let mut parsingtext = self.codeblock.to_string();
        let  toreturn: String;
        let mut scopecounter = 0;
        loop {
            let splitstr = split(&parsingtext, "{");

            if splitstr.len() > 1 {
                let isscope = split(&splitstr[splitstr.len() - 1], "}")[0];
                scopecounter +=1;
                let scopeid = scopecounter.to_string();
                let scopekey =" SCOPE ".to_string() + &scopeid;
                self.breakloop.push(false);
                //print(&format!("{}  -  {}",&scopekey,&isscope ),"b");
                let codevec = self.codetovector(&isscope);
                self.subblockmap.push(codevec);
                let toreplace = "{".to_owned() + &isscope + "}";
                parsingtext = Nstring::replace(&parsingtext, &toreplace, &scopekey);
            } else {
                toreturn = split(&splitstr[0], "}")[0].to_string();
                break;
            }
        }
        let codevec = self.codetovector(&toreturn);
        self.codeblockvector = codevec;
        self.codeblock = toreturn.to_string();

    }
    ///creates a Vector lines Vector words, used for parsing.
    pub fn codetovector(&mut self,code: &str) -> Vec<Vec<String>>{
        let mut codearray: Vec<Vec<String>> = Vec::new();
        let linearray: Vec<String> = code.split("\n").map(|s| s.to_string()).collect();
        for line in &linearray{
            let wordvec = line.trim().split(" ").map(|s| s.to_string()).collect();
            codearray.push(wordvec);
        }
        codearray
    }
    /// used for pre formatting,
    pub fn setcode(&mut self, codestring:String){
        //print(&codestring,"p");
        self.codeblock = codestring.to_string();
    }
    /// stores formatted code
     pub fn setcodevector(&mut self, codestring:Vec<Vec<String>>){
        self.codeblockvector = codestring;
    }
    ///gets the stringdata of a variable inside the block hashmap of a referenced variable
    pub fn getvarstring(&mut self,name:&str)->String{
        let get = self.variables.get_mut(name);
        if let Some(this) = get{
            //this.stringdata = "aaa".to_string();
            return this.stringdata.to_string();
        }
        else{
            let thisvar = NeoCatVar::new(name, "string");
            self.variables.insert(name.to_string(), thisvar);
            return "".to_string();
        }
    }
    ///sets the stringdata of a variable inside the block hashmap to a referenced variable
    pub fn setvarstring(&mut self,name:&str,data:&str){
        if let Some(this) = self.variables.get_mut(name){
            this.stringdata = data.to_string();
        }
        else{
            let mut thisvar = NeoCatVar::new(name, "string");
            thisvar.stringdata = data.to_string();
            self.variables.insert(name.to_string(), thisvar);
        }
    }
    ///copies a variable from the block for mutable purposes
    pub fn getvar(&mut self,name:&str)->NeoCatVar{
        if let Some(var) = self.variables.get_mut(name){
            return var.clone();
        }
        else{
            return NeoCatVar::new("error","string");
        };
    }
    pub fn getvarreference(&mut self,name:&str)->Option<&mut NeoCatVar>{
        if let Some(var) = self.variables.get_mut(name){
            return Some(var);
        }
        else{
            return None;
        };
    }
    pub fn setvar(&mut self,name:&str,var:NeoCatVar){
        if self.variableindex.contains(&name.to_string()) == false{
            self.variableindex.push(name.to_string());
        }
        self.variables.insert(name.to_string(), var);

    }
}
/// implement this to add new NeoCat rust functions and bind them
pub trait NeoCatStructBinding {
    fn neocat_exec(&mut self,tocall:&str,args: &Vec<NeoCatVar>) -> NeoCatVar;
}
/// Temp struct for executing scopes, disposes when done. ( garbage collector)
pub struct NeoCatScriptScope{
    name: String,
    pub  classrefs: Vec<String>,
    // pub variables:HashMap<String,NeoCatVar>,
}

impl NeoCatScriptScope{
    pub fn new(name:String)->NeoCatScriptScope{
        NeoCatScriptScope{
            name: name,
            classrefs: Vec::new(),
            //variables: HashMap::new(),
        }
    }
    pub fn name(&self) -> String{
        return self.name.to_string();
    }


}

// enum NeoCatVarType{
//     String,
//     Array,
// }
/// NeoCat Variable struct
pub struct NeoCatVar{
    pub name: String,// in string
    pub vartype: String,// in string
    //checktype: NeoCatVarType,// in string
    pub stringdata: String,
    pub stringvec: Vec<String>,
}
/// Variable struct holds the neocat datatypes and data
impl NeoCatVar{
    pub fn new(name:&str,vartype:&str) -> NeoCatVar{
        NeoCatVar{
            name: name.to_string(),
            vartype: vartype.to_string(),
            //checktype: NeoCatVarType::String,
            stringdata: "".to_string(),
            stringvec:Vec::new(),
        }
    }
    /// copies so they can be included to other blocks
    pub fn clone(&mut self)->NeoCatVar{
        NeoCatVar{
            name: self.name.to_string(),
            vartype: self.vartype.to_string(),
            //checktype: self.checktype,
            stringdata:self.stringdata.to_string(),
            stringvec:self.stringvec.clone(),
        }
    }
    /// returns the string value of the variable
    pub fn getstring(&mut self) -> String{
        return self.stringdata.to_string();
    }
    pub fn getnumber(&mut self) -> u64{
        return self.stringdata.parse::<u64>().unwrap_or(0);
    }
    pub fn setstring(&mut self,newstring:&str){
        self.stringdata = newstring.to_string()
    }

    pub fn settype(&mut self,settype: &str){
        match settype {
            "string" => {
                self.vartype = "string".to_string();
                //self.checktype = NeoCatVarType::String;
            }
            "array" => {

                self.vartype = "array".to_string();
                //self.checktype = NeoCatVarType::Array;
            }
            _ => {
                self.vartype = "string".to_string();
                //self.checktype = NeoCatVarType::String;
            }
        };
    }
    // pub fn checktype(&mut self) ->String{
    //     return &self.checktype;
    // }
    fn debug(&mut self){
        println!("name:{} type:{} stringdata:{} vecdata:{}",self.name,self.vartype,self.stringdata,self.stringvec.join(" >--< "))
    }
}
/// NeoCat user scripted functions
pub struct NeoCatFunc{
    pub name: String,
    pub args:Vec<String>,
    pub codeblock:NeoCatCodeBlock,
}

impl NeoCatFunc{
    pub fn new(name:String,args:Vec<String>)->NeoCatFunc{
        NeoCatFunc{
            name: name.to_string(),
            args: args,
            codeblock: NeoCatCodeBlock::new(&name),
        }
    }
    pub fn clone(&mut self)-> NeoCatFunc{
        NeoCatFunc{
            name:self.name.to_string(),
            args:self.args.clone(),
            codeblock:self.codeblock.copy(),
        }
    }
}

pub trait NeoCatLexer{
    fn test(&self);
}

impl <'a>NeoCatLexer for NeoCat<'a>{
    fn test(&self){
        println!("test!");
    }
    // fn setcode(&mut self, name: &str, code: &str) {
    //     self.codeblockmap.insert(name.to_string(), code.to_owned());
    // }
    //  fn setcodevector(&mut self, name: &str, code: &str) {
    //     let mut codearray: Vec<Vec<String>> = Vec::new();
    //     let linearray: Vec<String> = code.split("\n").map(|s| s.to_string()).collect();
    //     for line in &linearray{
    //         let wordvec = line.split(" ").map(|s| s.to_string()).collect();
    //         codearray.push(wordvec);
    //     }
    //     self.codeblockvectormap.insert(name.to_owned(),codearray);
    // }
    //  fn getcodevector(&mut self, name: &str) -> Vec<Vec<String>> {
    //     let g = self.codeblockvectormap.get_key_value(name);
    //     let result = match g {
    //         None => {
    //             Vec::new()
    //         }
    //         Some((_i, k)) => {
    //             let result = k.to_owned();
    //             result
    //         }
    //     };
    //     result
    // }
    //  fn getcode(&mut self, name: &str) -> String {
    //     let g = self.codeblockmap.get_key_value(name);
    //     let result = match g {
    //         None => {
    //             String::from("")
    //         }
    //         Some((_i, k)) => {
    //             let result = k.to_owned();
    //             result
    //         }
    //     };
    //     result
    // }
}

pub struct NeoCatClass{
    pub name: String,
    pub index: Vec<String>,
    properties: HashMap<String,NeoCatVar>,
    pub functionindex: Vec<String>,
    pub functions: HashMap<String,NeoCatFunc>,
    pub parents: Vec<String>,
    pub children: Vec<String>,
    //pub codeblock: NeoCatCodeBlock,
}

impl NeoCatClass{
    pub fn new(name:&str) -> NeoCatClass{
        NeoCatClass{
            name: name.to_string(),
            index: Vec::new(),
            properties: HashMap::new(),
            functionindex: Vec::new(),
            functions: HashMap::new(),
            parents: Vec::new(),
            children: Vec::new(),
            //codeblock: NeoCatCodeBlock::new(),
        }
    }
    pub fn clone(&mut self) -> NeoCatClass{
        let mut this = NeoCatClass{
            name: self.name.to_string(),
            index: self.index.clone(),
            properties: HashMap::new(),
            functionindex:self.functionindex.clone(),
            functions:HashMap::new(),
            parents: Vec::new(),
            children: Vec::new(),
        };

        for xprop in self.index.clone(){
            this.setprop(&xprop, self.getprop(&xprop));
        };
        for xprop in self.functionindex.clone(){
            this.setfunc(&xprop, self.getfunc(&xprop));
        };

        this
    }
    pub fn copyto(&mut self, name:&str) -> NeoCatClass{
        let mut this = NeoCatClass{
            name: name.to_string(),
            index: self.index.clone(),
            properties: HashMap::new(),
            functionindex:Vec::new(),
            functions:HashMap::new(),
            parents: Vec::new(),
            children: Vec::new(),
        };

        for xprop in self.index.clone(){
            this.setprop(&xprop, self.getprop(&xprop));
        };
        for xprop in self.functionindex.clone(){
            this.setfunc(&xprop, self.getfunc(&xprop));
        };
        this.parents.push(self.name.to_string());
        self.children.push(this.name.to_string());

        this
    }
    /// expands a class with some other class.
    pub fn inherent(&mut self, fromclass:&mut NeoCatClass){
        for xprop in fromclass.index.clone(){
            self.setprop(&xprop, fromclass.getprop(&xprop));
        };
        for xprop in fromclass.functionindex.clone(){
            self.setfunc(&xprop, fromclass.getfunc(&xprop));
        };
        self.parents.push(fromclass.name.to_string());
        fromclass.children.push(self.name.to_string());
    }
    pub fn setprop(&mut self, name:&str,prop:NeoCatVar){
        if let Some(_) = self.properties.get_mut(name){
            self.properties.insert(name.to_string(),prop);
        }
        else{
            self.index.push(name.to_string());
            self.properties.insert(name.to_string(),prop);
        }
    }
    pub fn getprop(&mut self,name:&str) ->NeoCatVar{
        if let Some(this) = self.properties.get_mut(name){
            this.clone()
        }
        else{
            NeoCatVar::new(name,"string")
        }
    }
    pub fn removeprop(&mut self,name:&str){
        self.index.retain(|x| x != name);
        self.properties.remove(name);
    }
    pub fn removefunc(&mut self,name:&str){
        self.functionindex.retain(|x| x != name);
        self.functions.remove(name);
    }
    pub fn setfunc(&mut self, name:&str,prop:NeoCatFunc){
        if let Some(_) = self.functions.get_mut(name){
            self.functions.insert(name.to_string(),prop);
        }
        else{
            self.functionindex.push(name.to_string());
            self.functions.insert(name.to_string(),prop);

        }
    }
    pub fn getfunc(&mut self,name:&str) ->NeoCatFunc{
        if let Some(this) = self.functions.get_mut(name){
            return this.clone();
        }
        else{
            print(&format!("NeoCatClass: Cant get func [{}]",&name),"r");
            NeoCatFunc::new(name.to_string(),Vec::new())
        }
    }

}

//-------------------------------------- teststuff--------------------------------------
struct StructA {
    value: i32,

}
// impl StructA{
//     fn set(&mut self){
//
//         self.value = 123456;
//     }
// }
impl NeoCatStructBinding for StructA {
    fn neocat_exec(&mut self,_tocall:&str,_args: &Vec<NeoCatVar>) -> NeoCatVar {

        //println!("StructA value: {} arg{}", self.value,args.join(","));

        self.value = 12;

        return NeoCatVar::new("nothing","string");
    }

}

struct StructB {
    value: String,
}

impl NeoCatStructBinding for StructB {
    fn neocat_exec(&mut self,_tocall:&str,_args: &Vec<NeoCatVar>) -> NeoCatVar{
        print(&self.value,"g");
        return NeoCatVar::new("nothing","string");
    }
}


pub fn testmain() {

    let mut neo = NeoCat::new();



    let mut a = StructA { value: 18 };
    let mut b = StructB { value: "Hello".to_string() };

    a.value = 222;
    neo.insertstruct("a", &mut a);
    neo.insertstruct("b", &mut b);
    let mut nstring = Nstring{};
    neo.insertstruct("string", &mut nstring);
    let mut ntimer = Ntimer{};
    neo.insertstruct("timer", &mut ntimer);
    neo.parsefile("./test.nc");
    let mut test = NeoCatVar::new("testvar","string");
    test.debug();
    if let Some(item) = neo.ruststructs.get_mut("a"){
        //item.value = 1;
        item.neocat_exec("", &Vec::new());

    }
    if let Some(item) = neo.ruststructs.get_mut("a") {
        item.neocat_exec("", &Vec::new());
    }
    let mut thisclass = neo.getclass("test");
    let thisprop = thisclass.getprop("name");

    print(&thisprop.stringdata,"g");
    print("testmain() Coroutines moeten de ifscopes / subblocken goed meekrijgen nog , \n -> verder lekker bezig pik!\n ","g");

    neo.executecoroutines();
    print("loop2","y");
    neo.executecoroutines();
    neo.executecoroutines();
for _ in 0..100{

        neo.executecoroutines();
    }
    print("loop4","y");
    neo.executecoroutines();
    // loop {
    //     if neo.coroutines.len() > 0 {
    //         neo.executecoroutines();
    //     }else{
    //         break;
    //     }
    // }
}
