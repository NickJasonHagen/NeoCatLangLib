use std::{f64, usize};

use crate::*;

pub trait NeoCatFormatter{
    fn parsefile(&mut self,file:&str) -> NeoCatVar;
    fn parsecode(&mut self,code:&str,name:&str) -> NeoCatVar;
    fn extract_scope(&mut self,filedata:&str) -> String;
    fn stripcomments(&mut self,filedata:&str) -> String;
    fn stringextract(&mut self,filedata:&str) -> String;
    fn thread_scopeextract(&mut self,codefiledata:&str,neocatscript: &mut NeoCatScriptScope) -> String;
    fn class_scopeextract(&mut self,codefiledata:&str,neocatscript: &mut NeoCatScriptScope) -> String;
    fn func_scopeextract(&mut self,codefiledata:&str,onclass:&str) -> String;
    fn executeblock(&mut self,block:&mut NeoCatCodeBlock) -> NeoCatVar;
    fn executescope(&mut self,codevec:&Vec<Vec<String>>, block:&mut NeoCatCodeBlock) -> Option<NeoCatVar>;
    fn executesubscope(&mut self,line:&Vec<String>, block:&mut NeoCatCodeBlock) -> Option<NeoCatVar>;
    fn executeline(&mut self,line:&Vec<String>, block:&mut NeoCatCodeBlock) -> NeoCatVar;
    fn executeword(&mut self,word:&str,block:&mut NeoCatCodeBlock) -> NeoCatVar;
    fn setdefiningword(&mut self,line:&str,equalsfrom:NeoCatVar,block:&mut NeoCatCodeBlock);
    fn checkwordtype(&mut self,word:&str) -> String;
    fn checkwordvalue(&mut self,word:&str,block:&mut NeoCatCodeBlock) -> NeoCatVar;
    fn parse_and_check_statements(&mut self,words:&Vec<String>,block:&mut NeoCatCodeBlock) -> bool;
    fn check_statement(&mut self,a:&str,b:&str,c:&str,block:&mut NeoCatCodeBlock) -> bool;
    fn runmath(&mut self,splitline:&Vec<String>,index:usize,block:&mut NeoCatCodeBlock) -> NeoCatVar;
    fn math(&mut self,a:&str,method:&str,b:&str,block:&mut NeoCatCodeBlock) -> NeoCatVar;
    fn matchscope(&mut self,tomatch: &str,subscope:usize,block:&mut NeoCatCodeBlock) -> NeoCatVar;
    fn executecoroutines(&mut self);
}

impl <'a>NeoCatFormatter for NeoCat<'a>{
    fn parsefile(&mut self,file:&str) -> NeoCatVar{
        let filedata = "\n".to_string() + &read_file_utf8(file);
        return self.parsecode(&filedata,&file);
    }
    fn parsecode(&mut self,code:&str,name:&str) -> NeoCatVar{
        let mut thiscodescope = NeoCatScriptScope::new(name.to_string());
        let mut thiscodeblock = NeoCatCodeBlock::new(name);
        let mut filedata = "\n".to_string() + &code;
        filedata = self.stripcomments(&filedata);
        filedata = self.stringextract(&filedata);
        filedata = "\n".to_string() + &filedata;
        //println!("code[{}]",&filedata);
        filedata = self.thread_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.class_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.func_scopeextract(&filedata,"");
        thiscodeblock.codeblock = filedata.to_string();
        //println!("lasblock:{}",filedata);
        let mut initblock = NeoCatCodeBlock::new("start");
        initblock.setcode(filedata);
        initblock.formatblock();
        return self.executeblock(&mut initblock);
    }
    fn executecoroutines(&mut self){
        for xroutine in self.coroutines.clone(){
            let  mut thisblock = self.getblock(&xroutine);
            self.executeblock(&mut thisblock);
            self.codeblocks.insert(xroutine,thisblock);
        }
    }
    /// entree point for executing a new block
    fn executeblock(&mut self,blocki:&mut NeoCatCodeBlock) -> NeoCatVar{
        //let mut blockref = blocki.codeblock;
        let  block = blocki.codeblockvector.clone();
        for lines in block{
            if lines[0] != "" {
                let returnvar = self.executeline(&lines, blocki);
                if returnvar.name == "return" {
                    //print(&format!("final execblock: returing [{}]",&returnvar.stringdata),"g");
                    return returnvar;
                }
            }
        }
        let returnvar = NeoCatVar::new("return","string");
        returnvar
    }
    /// recursively used to parse subscopes and jump blocks
    fn executescope(&mut self,block:&Vec<Vec<String>>,blocki: &mut NeoCatCodeBlock)->Option<NeoCatVar>{
        for lines in block{
            if lines[0] != "" {
                let result = self.executeline(&lines, blocki);
                if result.name == "return" {
                    //print(&format!("return  to execblock from execscope :[{}] ->[{}]",&line[0], &result.stringdata),"y");
                    return Some(result);
                }
            }
        }
        return None;
    }
    fn executesubscope(&mut self,line:&Vec<String>, block:&mut NeoCatCodeBlock) -> Option<NeoCatVar> {

        let toreturn:Option<NeoCatVar> = None;
        if line.len() > 1{
            block.insubblock = line[line.len()-1].parse::<usize>().unwrap_or(0);
            block.breakloop.push(false);
            if block.insubblock < block.breakloop.len() {
                block.breakloop[block.insubblock] = false;
            }
            let  index = block.insubblock-1;
             let sublen = block.subblockmap.len();
            // if sublen <= index {
            //     index = sublen-1;
            // }
            if sublen > 1 {
            if let Some(result) = self.executescope(&block.subblockmap[index].clone(),block){
                if result.name == "return"{
                    return Some(result);
                }
            }
            }

        }else{
            println!("cant execute subscope [{}] on line [{}]",&line[line.len()-1] , &line.join(" ")) ;
        }
        return toreturn;
    }
    /// interpretation of a line, has the words formatted in a vector
    fn executeline(&mut self,line:&Vec<String>,block:&mut NeoCatCodeBlock) ->NeoCatVar{
        let len = line.len();
        //println!("execline:{}",&line.join(" "));
        // match len {
        //     1 => {
        //         if line[0] == "break"{
        //             block.breakloop[block.inloop] = true;
        //             let mut retvar = NeoCatVar::new("return","string");
        //             retvar.stringdata = "break".to_string();
        //             return retvar;
        //         }
        //         if self.checkwordtype(&line[0]) == "variable" ||  self.checkwordtype(&line[0]) == "global" || self.checkwordtype(&line[0]) == "property"{
        //             let mut thisret = self.executeword(&line[0], block);
        //             thisret.name = "return".to_string();
        //         }
        //         return self.executeword(&line[0], block);
        //     }
        //     2 =>{
        //
        //     }
        //     3 =>{
        //
        //     }
        //     _ => {:arm
        //
        //     }
        // }
        if len == 1 {

            if line[0] == "break"{
                block.breakloop[block.inloop] = true;
                let mut retvar = NeoCatVar::new("return","string");
                retvar.stringdata = "break".to_string();
                return retvar;
            }
            if self.checkwordtype(&line[0]) == "variable" ||  self.checkwordtype(&line[0]) == "global" || self.checkwordtype(&line[0]) == "property" || self.checkwordtype(&line[0]) == "bool" || self.checkwordtype(&line[0]) == "static" || self.checkwordtype(&line[0]) == "number" || self.checkwordtype(&line[0]) == "macro" || self.checkwordtype(&line[0]) == "array"{
                let mut thisret = self.executeword(&line[0], block);
                //println!("returning 1 word: {} w : [{}]",&line[0],&thisret.stringdata);
                thisret.name = "return".to_string();
                return thisret;
            }
            return self.executeword(&line[0], block);

        }
        else if len > 1 {
            match line[0].as_str(){
                "return" => {
                    let mut var = self.executeword(&line[1], block);
                    //print(&format!("execline return [{}] -> [{}]",&var.name,&var.stringdata),"r");
                    var.name = "return".to_string();
                    return var;
                }
                "coroutine" => {

                    let coname = "coroutine_".to_string() + &self.executeword(&line[1], block).stringdata;
                    let mut coroutineblock = NeoCatCodeBlock::new(&coname);
                    for xvar in block.variableindex.clone(){
                        coroutineblock.setvar(&xvar, block.getvar(&xvar));
                    }
                    let mut selfvar = NeoCatVar::new("self","string");
                    selfvar.stringdata = coname.to_string();
                    coroutineblock.setvar("self", selfvar);
                    let scopeid = line[line.len()-1].parse::<usize>().unwrap_or(0);
                    let getcode = block.subblocktostring(scopeid);
                    //print(&scopeid.to_string(),"p");
                    //print(&getcode,"y");
                    coroutineblock.staticstrings = block.staticstrings.clone();
                    coroutineblock.setcode(getcode);
                    coroutineblock.formatblock();
                    coroutineblock.subblockmap = block.subblockmap.clone();
                    self.addcoroutine(&coname);
                    self.codeblocks.insert(coname,coroutineblock );
                    return NeoCatVar::new("coroutine","string");

                }
                "break" =>{
                    let tobreak = self.executeword(&line[1], block).stringdata;
                    self.removecoroutine(&tobreak);
                    return NeoCatVar::new("coroutine","string");
                }

                "SCOPE" =>{
                        if let Some(mut result) = self.executesubscope(&line, block){
                            if result.name == "return" {
                                return result.clone();
                            }
                        }
                }
                "match" =>{
                    let tomatch = self.executeword(&line[1], block);
                    let mut thisvar =  self.matchscope(&tomatch.stringdata,line[line.len()-1].parse::<usize>().unwrap_or(0)-1, block);
                    thisvar.name = "match".to_string();
                    return thisvar;
                }
                "if" => {
                    let statementresult = self.parse_and_check_statements(&line, block);
                    if statementresult{
                        block.ifset(statementresult);
                        block.ifup();
                        if let Some(mut result) = self.executesubscope(&line, block){
                            block.ifdown();
                            if result.name == "return" {
                                return result.clone();
                            }
                        }
                        block.ifdown();
                    }
                }
                "elseif" => {
                    if block.ifscopes[block.ifscopedepth] == false{
                        let statementresult = self.parse_and_check_statements(&line, block);
                        if statementresult{
                            block.ifset(statementresult);
                            block.ifup();
                            if block.ifscopes[block.ifscopedepth] == false{
                                if let Some(mut result) = self.executesubscope(&line, block){
                                    block.ifdown();
                                    if result.name == "return" {
                                        return result.clone();
                                    }
                                }
                                block.ifdown();
                            }
                        }
                    }
                }
                "else" =>{
                    if block.ifscopes[block.ifscopedepth] == false{
                        if let Some(mut result) = self.executesubscope(&line, block){
                            if result.name == "return" {
                                return result.clone();
                            }
                        }
                    }
                }
                "loop" =>{
                    block.inloop +=1;
                    block.breakloop[block.inloop] = false;
                    loop {
                        if let Some(mut result) = self.executesubscope(&line, block){
                            if result.name == "return"{
                                if result.stringdata == "break"{
                                    block.breakloop[block.inloop] = true;
                                    block.inloop -=1;
                                    break;
                                }
                                else{
                                    block.inloop -=1;
                                    return result.clone();
                                }
                            }
                        }
                        if block.breakloop[block.inloop] {
                            block.breakloop[block.inloop] = true;
                            block.inloop -=1;
                            break;
                        }
                    }
                    return NeoCatVar::new("loopdone","")
                }
                "spawnthread" => {
                    let fromthreadname = "thread_".to_string()+&line[2];
                    let threadname = "thread_".to_string()+&self.executeword(&line[1], block).stringdata;
                    let mut threadcode = self.getblock(&fromthreadname);
                    threadcode.codeblock = "\n".to_string() + &threadcode.codeblock;
                    //print(&format!("thread [{}] threadcode [{}]", &threadname,&threadcode.codeblock),"p");
                    let (main_to_worker_tx, main_to_worker_rx) = mpsc::channel();
                    let (worker_to_main_tx, worker_to_main_rx) = mpsc::channel();

                    //vmap.threads.insert("nscriptmainthread".to_string(),(main_to_worker_tx, main_to_worker_rx));

                    self.threadsreceiver.insert(threadname.to_string(),worker_to_main_rx);

                    self.threadssenders.insert(threadname.to_string(),main_to_worker_tx);
                        let worker_to_main_tx = Arc::new(Mutex::new(worker_to_main_tx));
                    //println!("THREAD:[{}] [{}]",&line[1],line.join(" "));
                    //threadstruct.codeblocks.insert("main".to_string(),threadcode);
                    //print(&format!("threadcode [{}]",&threadcode.codeblock),"g");
                    //

                    thread::spawn(move || {

                        let mut threadstruct = NeoCat::new();
                        threadstruct.parsecode(&threadcode.codeblock.to_string(), &threadname);
                        loop {
                            threadstruct.executecoroutines();
                            if threadstruct.coroutines.len() < 1 {
                                //print("breaking routine thread","r");
                                break;
                            }
                            let sender = worker_to_main_tx.lock().unwrap();
                            let received_message: NeoCatVar = match main_to_worker_rx.try_recv(){
                                Ok(rmsg) => {
                                    rmsg
                                }
                                Err(_)=>{
                                   NeoCatVar::new("error","string")
                                }
                            };
                            if received_message.name != "error"{
                                //println!("thread received:{}",received_message);

                                //println!("received: {}",received_message.stringdata);
                                threadstruct.setglobal("$received",received_message);
                                let ncfunc = "threadreceive($received)";
                                let mut funcblock = threadstruct.getblock("threadreceive");
                                let ncreturn = threadstruct.executeword(&ncfunc,&mut funcblock);
                                match sender.send(ncreturn){
                                    Ok(_)=>{},
                                    Err(_)=>{},

                                };
                                threadstruct.setglobal("$received", NeoCatVar::new("$received","string"));

                            }
                        }
                    });
                    return NeoCatVar::new("THREAD","")
                }
                "for" =>{
                    if len > 4{
                        if line[2] == "in" {

                            let arrayvar = self.executeword(&line[3], block);
                            let mut iteratevar = NeoCatVar::new(&line[1],"string");
                            for index in arrayvar.stringvec {
                                iteratevar.stringdata = index.to_string();
                                block.setvar(&line[1], iteratevar.clone());
                                if let Some(mut result) = self.executesubscope(&line, block){

                                    if result.name == "return"{
                                        if result.stringdata == "break"{
                                            return result.clone();
                                        }
                                    }
                                }
                            }
                            return iteratevar;
                        }
                        if line[2] == "to" {
                            let splitfrom = split(&line[1],"=");

                            let mut start = 0;
                            if splitfrom.len() > 1{
                                start = self.executeword(&splitfrom[1],block).stringdata.parse::<usize>().unwrap_or(0);
                            }
                            let mut iteratevar = NeoCatVar::new(&splitfrom[0],"string");
                            for index in start..line[3].parse::<usize>().unwrap_or(0)+1 {
                                iteratevar.stringdata = index.to_string();
                                block.setvar(&splitfrom[0], iteratevar.clone());
                                if let Some(mut result) = self.executesubscope(&line, block){

                                    if result.name == "return"{
                                        if result.stringdata == "break"{
                                            return result.clone();
                                        }
                                    }
                                }
                            }
                            return iteratevar;
                        }
                    }
                }
                _ =>{

                }
            }
            match line[1].as_str() {

                "=" => {
                    let mut equalsfrom:NeoCatVar = NeoCatVar::new(&line[0],"string");
                    if len == 3{
                        equalsfrom = self.executeword(&line[2], block);
                    }
                    if len > 3 {
                        match line[2].as_str(){
                            "cat" => {
                                //print("doing cat","b");
                                let mut toadd = "".to_string();
                                equalsfrom = NeoCatVar::new(&line[0],"string");
                                for xadd in 2..line.len(){
                                    let this = self.executeword(&(line[xadd].to_string()),block);
                                    toadd = toadd.to_string() + &this.stringdata.to_string();
                                }
                                equalsfrom.stringdata = toadd.to_string();
                            }
                            "match" => {
                                let tomatch = self.executeword(&line[3], block);
                                equalsfrom =  self.matchscope(&tomatch.stringdata,line[line.len()-1].parse::<usize>().unwrap_or(0)-1, block);
                                //println!("matchfrom: [{}]:[{}]", &equalsfrom.name,&equalsfrom.stringdata);
                                equalsfrom.name = line[0].to_string();
                            }
                            _ =>{
                                if line[3] == "+" || line[3] == "-" || line[3] == "/"  || line[3] == "*"{
                                    //print("doing math","b");
                                    let mut stringvec:Vec<String> = Vec::new();
                                    //stringvec.push("".to_string());
                                    for x in 2..line.len(){
                                        stringvec.push(line[x].to_string());
                                    }
                                    equalsfrom = self.runmath(&stringvec, 0, block);
                                }
                                if line[3] == "!=" || line[3] == ">=" || line[3] == "<="  || line[3] == "==" || line[3] == "<" || line[3] == ">"{
                                    print("doing statement","b");
                                    let mut stringvec:Vec<String> = Vec::new();
                                    stringvec.push("if".to_string());
                                    for x in 2..line.len(){
                                        stringvec.push(line[x].to_string());
                                    }
                                    stringvec.push("".to_string());
                                    stringvec.push("".to_string());
                                    equalsfrom.stringdata = self.parse_and_check_statements(&stringvec, block).to_string();
                                }
                            }
                        }
                    }
                    self.setdefiningword(&line[0], equalsfrom, block);
                }
                ":" =>{
                    let mut thisclass = line[0].to_string();
                    if self.checkwordtype(&line[0]) == "reflection"{
                        thisclass = self.executeword(&line[0], block).stringdata;
                    }
                    let mut fromclass = line[2].to_string();
                    if self.checkwordtype(&line[2]) == "reflection"{
                        fromclass = self.executeword(&line[2], block).stringdata;
                    }

                    let mut parentclass = self.getclass(&fromclass);
                    if let Some(class) = self.getclassref(&thisclass){
                        class.inherent(&mut parentclass);
                        //println!(" class:[{}] extended to [{}]",fromclass,thisclass);
                    }else{
                        let class = parentclass.copyto(&thisclass);

                        self.insertclass(&thisclass, class);
                        //println!(" class:[{}] created as a copy from, [{}]",thisclass,fromclass);
                    }
                }
                "!" =>{ // new printing syntax ===> var !
                    let thisword = self.executeword(&line[0], block);
                    print(&format!("-{}-->{}",&line[0], &thisword.stringdata),"bp");
                }
                "!!" =>{ // new printing syntax ===> var !
                    let thisword = self.executeword(&line[0], block);
                    print(&format!("-{}-->{}",&line[0], &thisword.stringdata),"br");
                }
                "&=" =>{

                    if len > 2 {
                        //let mut fromvar = self.executeword(&line[2], block);
                        let mut equalsfrom = self.executeword(&line[0], block);
                        //let mut toadd = equalsfrom.stringdata.to_string();
                        for xadd in 2..line.len(){
                            let this = self.executeword(&(line[xadd].to_string()),block);
                           equalsfrom.stringdata = equalsfrom.stringdata + &this.stringdata.to_string();
                        }
                        self.setdefiningword(&line[0], equalsfrom, block);
                    }
                }
                "++" =>{
                    let mut onvar = self.executeword(&line[0], block);
                    onvar.stringdata = (onvar.getnumber() + 1).to_string();
                    self.setdefiningword(&line[0], onvar, block);

                }
                "+=" =>{
                    let mut onvar = self.executeword(&line[0], block);
                    let mut total = onvar.getnumber();
                    for x in 2..len{
                        let mut onvar = self.executeword(&line[x], block);
                        total += onvar.getnumber();
                    }
                    onvar.stringdata = total.to_string();
                    self.setdefiningword(&line[0], onvar, block);
                }
                "--" =>{
                    let mut onvar = self.executeword(&line[0], block);
                    onvar.stringdata = (onvar.getnumber() - 1).to_string();
                    self.setdefiningword(&line[0], onvar, block);

                }
                "-=" =>{
                    let mut onvar = self.executeword(&line[0], block);
                    let mut total = onvar.getnumber();
                    for x in 2..len{
                        let mut onvar = self.executeword(&line[x], block);
                        total -= onvar.getnumber();
                    }
                    onvar.stringdata = total.to_string();
                    self.setdefiningword(&line[0], onvar, block);
                }
                _ =>{

                }
            }
        };
        NeoCatVar::new("","string")
    }
    fn setdefiningword(&mut self,word:&str,equalsfrom: NeoCatVar, block:&mut NeoCatCodeBlock){
        let  vartype = self.checkwordtype(&word);
        //let  varname = line[0].to_string();

        //print(&varname,"y" );
        //print(&vartype,"br" );
        match vartype.as_str(){

            "global" => {
                //print(&format!("declaring global {} with {}",&varname,&equalsfrom.stringdata),"m");
                self.setglobal(&word, equalsfrom);
            }
            "property" => {

                let splitword = split(&word,".");
                if splitword.len() > 1{
                    let mut classname = splitword[0].to_string();
                    let trimmedname = Nstring::trimleft(&splitword[0],1);
                    if Nstring::fromleft(&classname, 1) == "*" {
                        classname = self.executeword(&trimmedname,block).stringdata.to_string();

                    }
                    let mut propname = splitword[1].to_string();
                    let trimmedprop = Nstring::trimleft(&splitword[1],1);
                    if  Nstring::fromleft(&propname, 1)  == "*" {
                        propname = self.executeword(&trimmedprop,block).stringdata.to_string();

                    }
                    //let propname = self.executeword(splitword[1],block);

                    //print(&classname,"b");
                    //print(&propname,"bb");
                    if let Some(thisclass) = self.getclassref(&classname){
                        //print(&format!("declaring class [{}] property [{}] with [{}]",&classname,&propname,&equalsfrom.stringdata),"g");
                        thisclass.setprop(&propname, equalsfrom);
                    }
                    else{
                        let mut newclass = NeoCatClass::new(&classname);
                        newclass.setprop(&propname, equalsfrom);
                        self.insertclass(&classname,newclass);
                        //print(&format!("new class spawned class [{}]",&classname),"br");
                    }
                }
            }
            "variable" => {


                block.setvar(&word, equalsfrom);
            }
            _ =>{

            }
        };

    }
    /// recursively used to parse a word ,include nested subfunctions in arguments etc
    fn executeword(&mut self,word:&str,block:&mut NeoCatCodeBlock) -> NeoCatVar {

        let prefix = self.checkwordtype(word);
        match prefix.as_str(){
            "static" =>{
                let mut var = NeoCatVar::new("static","string");
                var.stringdata = block.staticstrings[Nstring::trimleft(word, 1).parse::<usize>().unwrap_or(0)].to_string();
                return var;
            }
            "nestedfunction" =>{

                let mut resultstring = word.to_string();
                let mut packed: String;
                let mut subfunction: String;

                loop {
                    // get the last find in the string using (
                    let splitstr = split(&resultstring, "(");
                    // make sure its inside the main function so bigger>2
                    if splitstr.len() > 2 {
                        //take that substring and split up to the first )
                        let splitscope = split(&splitstr[splitstr.len() - 1], ")");
                        if splitscope.len() > 0 {
                            // important one, if a variable or string is infron it
                            // messes up the syntax so we split using comma
                            let splitargus = split(&splitstr[splitstr.len() - 2], ",");
                            // here we set thisfnname to the last part of the comma split
                            let thisfnnamefix = splitargus[splitargus.len() - 1]; // make sure the function
                            // here we check if the function given is reflected if so we evaluate the value of
                            // the var and executre the function of the data from that var as a string
                            if Nstring::fromleft(&splitstr[splitstr.len() - 2], 1) == "*" {
                                let splitdot = split(&thisfnnamefix,".");
                                if splitdot.len() > 1 {

                                    let mut part1 = splitdot[0].to_string();
                                    let mut part2 = splitdot[1].to_string();
                                    if Nstring::fromleft(&splitdot[0], 1) == "*"{
                                        part1 = self.executeword(&Nstring::trimleft(&splitdot[0], 1), block).stringdata;
                                    }

                                    if Nstring::fromleft(&splitdot[1], 1) == "*"{
                                        part2 = self.executeword(&Nstring::trimleft(&splitdot[1], 1), block).stringdata;
                                    }
                                    let thisfnnamefix2 = part1.to_string() + "." + &part2;
                                    //print(&thisfnnamefix2,"r");
                                    subfunction = "".to_owned() + &thisfnnamefix2
                                        + "(" + &splitscope[0]+ ")";

                                }else{
                                    subfunction = "".to_owned()
                                        + &self.executeword(&Nstring::replace(&thisfnnamefix, "*", ""),block).stringdata
                                        + "(" + &splitscope[0]+ ")";

                                }
                            } else {
                                // if its a normal funcion we run it.
                                subfunction = "".to_owned() + &thisfnnamefix + "(" + &splitscope[0] + ")";
                            }
                            // here we evaluate the none function types.
                            packed = "^".to_owned() + &string_to_hex(&self.executeword(&subfunction, block).stringdata);
                        } else {
                            // this also evaluates variables macros strings etc
                            subfunction = "".to_owned() + &splitscope[0]; //&splitstr[splitstr.len()-1];
                            packed = "^".to_owned() + &string_to_hex(&self.executeword(&splitscope[0], block).stringdata);
                        }
                        let mut reflect = false;
                        if splitscope.len() > 0 {
                            // so this replaces the evaluated values in the word's() when
                            // its all done it will return 1 function to parseline() wich be used to set the
                            // variable
                            if Nstring::fromleft(&splitstr[splitstr.len() - 2], 1) == "*" {
                                subfunction = "".to_owned() + &splitstr[splitstr.len() - 2] + "(" + &splitscope[0] + ")";
                                resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
                                reflect = true
                            }
                        }
                        if reflect == false {
                            // very important! this reforms the strings till its made back to 1 function with
                            // all evaluated data types. when this is done theres no double (( )) insde the
                            // code and this function will exit and return the 1-function to parse_line()
                            resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
                        }
                    } else {
                        break;
                    }
                }
                return self.executeword(&resultstring, block);

            }
            "hexstring" =>{
                let mut thisvar = NeoCatVar::new("hexstring","string");
                thisvar.stringdata = hex_to_string(&Nstring::trimleft(word, 1));
                return thisvar;
            }
            "macro" =>{
                match word{
                    "@emptystring" | _ =>{
                        let mut var = NeoCatVar::new("macro","string");
                        var.stringdata = "".to_string();
                        return var;
                    }
                }
            }
            "global" => {
                return self.getglobal(&word).clone();
            }
            "variable"=>{
                return block.getvar(word).clone();
            }
            "property"=>{
                let wordsplit = split(&word,".");
                if wordsplit.len() > 1{
                    let mut cname = wordsplit[0].trim().to_string();
                    let mut pname = wordsplit[1].trim().to_string();
                    if Nstring::fromleft(&wordsplit[0], 1) ==  "*"{
                        //cname = block.getvar(&Nstring::trimleft(&wordsplit[0], 1)).stringdata ;
                        cname = self.executeword(&wordsplit[0], block).stringdata;

                    }
                    if Nstring::fromleft(&wordsplit[1], 1) ==  "*"{
                        pname = self.executeword(&Nstring::trimleft(&wordsplit[1], 1),block).stringdata ;
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname);
                    }else{

                        print(&format!("word is a prop but theres no class on cname [{}] pname[{}]",&cname,&pname),"r");
                    }
                }
            }
            "number" | "bool" =>{
                let mut newvar = NeoCatVar::new(word,"string");
                newvar.setstring(&word);
                return newvar;
            }
            "function" => {
                let mut args:Vec<String> = Vec::new();
                let splitfunc = split(&split(&word,"(")[0],".");
                let getargs = Nstring::stringbetween(&word, "(", ")");
                let givenargs = split(&getargs,",");
                //let  funcname = splitfunc[0].to_string();
                let functiontocall:String;
                let mut getblock =  NeoCatCodeBlock::new("");

                let mut i = 0;
                if splitfunc.len() > 1 {
                    let mut classname = splitfunc[0].to_string();
                    if Nstring::fromleft(&splitfunc[0],1) == "*"{
                        classname = self.executeword(&Nstring::trimleft(&splitfunc[0],1), block).stringdata;
                    }
                    let mut funcname = splitfunc[1].to_string();

                    if Nstring::fromleft(&splitfunc[1],1) == "*"{

                        funcname = self.executeword(&Nstring::trimleft(&splitfunc[1],1), block).stringdata;
                    }
                    //functiontocall = classname.to_string() + "." + &funcname;
                    args = Vec::new();
                    if let Some(class) = self.getclassref(&classname){
                        let mut thisfunc = class.getfunc(&funcname);
                        args = thisfunc.args.clone();
                        //print(&args.join("]["),"b");
                        getblock = thisfunc.codeblock.copy();
                    }
                }
                else{
                    if Nstring::instring(&splitfunc[0], "::") {
                        let splitstruct = split(&splitfunc[0],"::");
                        if splitstruct[0] == "threadsend"{
                            let thisthread = self.executeword(splitstruct[1], block);
                            let  get = self.executeword(&givenargs[0],block);

                            //println!("sending:{} to thread[{}]",&get.stringdata,&thisthread.stringdata);
                            return self.threadsend(&thisthread.stringdata, get);
                        }
                        else{
                            let mut argvarvec :Vec<NeoCatVar> = Vec::new();
                            for xarg in givenargs.clone(){
                                if givenargs.len() > i{
                                    let mut get = self.executeword(&givenargs[i],block);
                                    //print(&xarg,"y");
                                    get.name = xarg.to_string();
                                    argvarvec.push(get);
                                }
                                i +=1;
                            }

                            //print(&format!("userstruct::[{}]::[{}]",&splitstruct[0],&splitstruct[1]),"r");
                            if let Some(userstruct) = self.ruststructs.get_mut(splitstruct[0]){
                                return userstruct.neocat_exec(splitstruct[1], &argvarvec);
                            }
                        }

                    }

                    let mut funcname = splitfunc[0].to_string();
                    if Nstring::fromleft(&splitfunc[0],1) == "*"{
                        funcname = self.executeword(&Nstring::trimleft(&splitfunc[0],1), block).stringdata;
                    }
                    if let Some(func) = self.functions.get_mut(&split(&funcname,"(")[0].to_string()){
                        args = func.args.clone();
                    }
                    else{
                        print(&format!("Cannot find root func:[{}]",&funcname),"y");
                    }
                    functiontocall = funcname.to_string();
                    getblock = self.getblock(&functiontocall);
                }
                for xarg in args{
                    if givenargs.len() > i{
                        let get = self.executeword(&givenargs[i],block);
                        //println!("this xarg [{}] = [{}]",&xarg,&get.stringdata);
                        getblock.setvar(&xarg,get);
                    }
                    i +=1;
                }
                let result = self.executeblock(&mut getblock);
                //print(&format!("func:[{}] ->{}[{}]",&word,&result.name,&result.stringdata),"g");
                return result;
                //return self.executeword(&splitfunc[0], block);
            }
            "reflection" =>{
                let toreflect = Nstring::trimleft(word, 1);
                let evaluated = self.executeword(&toreflect, block);

                //print(&format!("reflecting {} as {}",&toreflect,&evaluated.stringdata),"bp");
                return evaluated;
            }
            "array" =>{
                let mut returnvar = NeoCatVar::new("entree","string");
                let thisvar = self.executeword(split(word,"[")[0], block);
                let index = self.executeword(&Nstring::stringbetween(&word, "[", "]"),block).stringdata.parse::<usize>().unwrap_or(0);
                if thisvar.stringvec.len() > index{
                    returnvar.stringdata = thisvar.stringvec[index].to_string();
                }
                return returnvar;

            }
            "arraydeclaration" => {
                let mut thisarrayvar = NeoCatVar::new("array","array");
                let between = Nstring::trimright(&Nstring::trimleft(&word,1),1);
                let inarray = split(&between,",");
                for arrayitem in inarray{
                    thisarrayvar.stringvec.push(self.executeword(&arrayitem, block).stringdata);
                }
                return thisarrayvar
            }
            _ => {
                print(&format!("word {} cannot execute",&word),"r");

            }
        }

        let mut retvar = NeoCatVar::new("error","string");
        print(&format!("error from word {}",&word),"br");
        retvar.setstring("error");
        return retvar;
    }
    /// used to check what a word / type is
    fn checkwordtype(&mut self,word:&str) -> String{
        let splitsubs = split(&word,"(");
        if Nstring::instring(&word, "(") &&  Nstring::instring(&word, ")") {
            // if Nstring::instring(&split(&word,"(")[0],"."){
            //     return "classfunction".to_string();
            // }
            if splitsubs.len()>2{
                //print("nested","b");
                //print(&word,"b");
                //print(&splitsubs.len().to_string(),"bb");
                return "nestedfunction".to_string();
            }
            return "function".to_string();
        }
        if Nstring::instring(&word, ".") && Nstring::instring(&word, "(") == false{
            return "property".to_string();
        }
        if Nstring::fromleft(word, 1) != "[" && Nstring::instring(&word, "[") &&  Nstring::instring(&word, "]") {
            return "array".to_string();
        }
        if word.parse::<f64>().is_ok(){
            return "number".to_string();
        }
        if word == "true" || word == "false"{
            return "bool".to_string();
        }
        let prefix = Nstring::fromleft(word, 1);
        match prefix.as_str(){
            "$" => {
                return "global".to_string();
            }
            "[" =>{
                return "arraydeclaration".to_string();
            }
            "~" => {
                return "static".to_string();
            }
            "@" => {
                return "macro".to_string();
            }
            "*" => {
                return "reflection".to_string();
            }
            "^" => {
                return "hexstring".to_string();
            }
            _ => {
                return "variable".to_string();
            }
        }
    }
    fn checkwordvalue(&mut self,word:&str,block:&mut NeoCatCodeBlock) -> NeoCatVar{
        return block.getvar(&word);
    }
    /// extracts the scope from the codesheet
    fn extract_scope(&mut self,filedata: &str) -> String {
        let mut stack = Vec::new();
        let mut start = None;
        let mut end = None;
        let mut depth = 0;

        for (index, ch) in filedata.char_indices() {
            match ch {
                '{' => {
                    if stack.is_empty() {
                        start = Some(index);
                    }
                    stack.push(ch);
                    depth += 1;
                }
                '}' => {
                    stack.pop();
                    depth -= 1;
                    if stack.is_empty() && depth == 0 {
                        end = Some(index + 1);
                        break;
                    }
                }
                _ => {}
            }
        }

        match (start, end) {
            (Some(start), Some(end)) => filedata[start..end].to_string(),
            _ => String::new(),
        }
    }

    /// strips off all comments per lines and trims the lines.
    fn stripcomments(&mut self, filedata: &str) -> String {
        let lines = filedata.split("\n");
        let mut newcode = String::new();
        for line in lines {
            if line != "" {
                newcode = newcode + &split(&line, "//")[0].trim() + "\n";
            }
        }
        newcode
    }
    /// encodes the static strings, will later on be parsed per scope and set as variables.
    /// pre-formatting
    fn stringextract(&mut self,filedata : &str) -> String {
        let mut parsingtext = Nstring::replace(&filedata.to_string(), "\\\"", "#!@NSCRIPTQUOTE#@!");
        parsingtext = Nstring::replace(&parsingtext, "\"\"", "@emptystring");
        loop {
            let splitstr = Nstring::stringbetween(&parsingtext, "\"", "\"");
            if splitstr != "" {
                let packed = "^".to_owned()
                + &string_to_hex(&Nstring::replace(&splitstr, "#!@NSCRIPTQUOTE#@!", "\" "));
                let toreplace = "\"".to_owned() + &splitstr + "\"";
                parsingtext = Nstring::replace(&parsingtext, &toreplace, &packed);
            } else {
                break;
            }
        }
        parsingtext
    }
    fn thread_scopeextract(&mut self,codefiledata:&str,_scriptscope: &mut NeoCatScriptScope) -> String{

        let mut i = 0; //<-- serves to filter first split wich isnt if found but default.
        let mut parsecode = codefiledata.to_string();
        let threads = split(&codefiledata, "\nthread");
        for xthread in threads {
            if i > 0 {
                if xthread != "" {
                    let namepart = split(&xthread, "{")[0];
                    let name = split(&namepart, "|");
                    let thisname = name[0].trim();
                    let subblockraw = self.extract_scope(&xthread); // extract the thread scope between { }
                    let mut  codeblock = NeoCatCodeBlock::new(&thisname);
                    codeblock.setcode(subblockraw.clone());
                    self.codeblocks.insert("thread_".to_string() + &thisname, codeblock);
                    let toreplace = "thread".to_owned() + &namepart + &subblockraw;
                    //let replacefor = "THREAD".to_string() + &namepart;
                    if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {
                        parsecode = Nstring::replace(&parsecode,&toreplace, "");
                    }
                }
            }
            i += 1;
        }
        parsecode
    }
    /// parses the code for all class scopes, will subparse for all functions inside.
    /// executes the left over block during parsing. in-order
    fn class_scopeextract(&mut self,codefiledata:&str,scriptscope: &mut NeoCatScriptScope) -> String{

        let mut i = 0; //<-- serves to filter first split wich isnt if found but default.
        let mut parsecode = codefiledata.to_string();
        let classes = split(&codefiledata, "\nclass");
        for eachclass in classes {
            if i > 0 {
                //let code = parsecode.clone();//vmap.getcode(&parsecode);
                if eachclass != "" {
                    //let  oldself:  String;
                    let classnamepart = split(&eachclass, "{")[0];
                    let classname = split(&classnamepart, ":");

                    let thisclassname = classname[0].trim();
                    let mut thisclass: NeoCatClass;
                    if let Some(_) = self.getclassref(thisclassname){
                        //thisclass = this.clone();
                    }else{// insert new classes
                        thisclass = NeoCatClass::new(&thisclassname);
                        self.insertclass(&thisclassname, thisclass.clone());
                        thisclass.name = thisclassname.to_string();
                        scriptscope.classrefs.push(thisclassname.to_string());

                    }
                    let subblockraw = self.extract_scope(&eachclass); // extract the class scope between { }
                    // if let Some(this) = self.getclassref(thisclassname){
                    //     thisclass = this.clone();
                    // }
                    let mut subblock = subblockraw.clone();//self.extract_scope(&eachclass); // extract the class scope between { }
                    subblock = Nstring::replace(&subblock, "self.", "*self.");
                    subblock = self.func_scopeextract(&subblock, &thisclassname);
                    let mut selfvar = NeoCatVar::new("self","string");
                    selfvar.stringdata = thisclassname.to_string();
                    let mut codeblock = NeoCatCodeBlock::new(&thisclassname);
                    codeblock.setvar("self", selfvar);
                    codeblock.setcode(subblock.to_string());
                    codeblock.formatblock();
                    //self.insertclass(&thisclassname, NeoCatClass::new(&thisclassname));

                    //self.insertclass(&thisclassname,thisclass);
                    self.executeblock(&mut codeblock);
                    if classname.len() > 1{
                        //let mut this = self.getclass(thisclassname);
                        let mut fromclass = self.getclass(&classname[1].trim().to_string());
                        if let Some(thisclass) = self.getclassref(&classname[0].trim()) {
                            thisclass.inherent(&mut fromclass);
                        }

                    }

                    //println!("inserting class {}",classname[0].trim());
                    self.codeblocks.insert("class_".to_string() + &thisclassname, codeblock);

                    let toreplace = "class".to_owned() + &classnamepart + &subblockraw;
                    if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {
                        //print(&format!("toreplace={}",&toreplace),"y");
                        parsecode = Nstring::replace(&parsecode,&toreplace, "");
                    }

                }
            }


            i += 1;
        }
        parsecode
    }
    /// extracts all functions from the sheet, class is empty for plain functions.
    fn func_scopeextract(&mut self,filedata: &str,onclass:&str) -> String {
        let classes = split(&filedata, "\nfunc ");
        let mut parsecode = filedata.to_string();
        let mut i = 0;
        let mut arguments : Vec<String> = Vec::new();
        for eachclass in classes {
            if i > 0 {
                //let code = vmap.getcode(&internalcoderef);
                if eachclass.trim() != "" && Nstring::fromleft(&eachclass.trim(), 1) != "{" {
                    let firstline = split(&eachclass, "{")[0];
                    let funcname = split(&firstline, "(")[0].trim();
                    let  block = self.extract_scope(&eachclass);
                    let cleanblock = block.clone();
                    let argumentsraw = split(&firstline, "(");
                    if argumentsraw.len() > 1 {
                        let argumentsraw = split(&argumentsraw[1], ")");
                        //let splitarguments = split(&argumentsraw[0], ",");
                        arguments = argumentsraw[0].split(",").map(str::to_string).collect();
                    }

                    let toreplace = "func ".to_owned() + &split(&eachclass, "{")[0] + &cleanblock;

                    // set the modified code
                    //print(&format!("loading func:[{}] with args[{}]",&funcname.trim(),&arguments.join(",")),"bg");
                    if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {
                        parsecode = parsecode.replace(toreplace.trim(), "");
                        if onclass == "" {
                            let mut thisblock = NeoCatCodeBlock::new(&("".to_string()+&funcname));
                            thisblock.setcode(block.to_string());
                            thisblock.formatblock();
                            self.codeblocks.insert("".to_string()+&funcname, thisblock);
                            let  thisfunc = NeoCatFunc::new(funcname.to_string(),arguments.clone());
                            //print(&format!("added func [{}]",&funcname),"g");
                            self.functions.insert(funcname.to_string(),thisfunc);

                        } else {

                            let mut thisblock = NeoCatCodeBlock::new(&(onclass.trim().to_string()+"."+&funcname.trim()));
                            thisblock.setcode(block.to_string());
                            thisblock.formatblock();

                            let mut varself = NeoCatVar::new("self", "string");
                            varself.setstring(&onclass);
                            thisblock.setvar("self",varself);
                            //thisblock.variables.insert("self".to_string(),varself);
                            let mut thisfunc = NeoCatFunc::new(funcname.to_string(),arguments.clone());
                            let blockname = onclass.to_string() + "." + &funcname;
                            if let Some(thisclass) = self.getclassref(onclass){
                                print(&format!("added func [{}] to class [{}]",&funcname,&onclass),"g");

                                thisfunc.codeblock = thisblock;
                                thisclass.setfunc(&funcname.to_string(),thisfunc);
                            }else{

                                let mut thisclass = NeoCatClass::new(&onclass);
                                thisclass.setfunc(&funcname.to_string(),thisfunc);
                                self.codeblocks.insert(blockname.to_string(), thisblock);
                                self.insertclass(&onclass, thisclass);
                                print(&format!("spawned class [{}] and added [{}]",&onclass,&funcname),"y");
                            }
                        }
                    }

                }
            }
            i += 1;
        }
        parsecode
    }

 fn parse_and_check_statements(&mut self ,words: &Vec<String>,block:&mut NeoCatCodeBlock) -> bool {
    // this is how you parse a unknown lenght of statements
    // they can be mixed And/or
    // this function will return a bool.
    // -------------------------------------------------------------
    if words.len() < 4 {
        if words[0] == "if" || words[0] == "elseif" {

            return false; // Invalid syntax or empty statement
        }
    }

    let conditions = &words[3..words.len() - 2];
    let mut index = 1;
    let mut result = self.check_statement(&words[1], &words[2], &words[3],block);
    // if result{
    //     return result;
    // }
    while index + 4 < conditions.len() + 1 {
        let operator = conditions[index].as_str();
        let a = conditions[index + 1].as_str();
        let b = conditions[index + 2].as_str();
        let c = conditions[index + 3].as_str();
        if operator == "and" || operator == "&&" {
            result = result && self.check_statement(&a, &b, &c,block);
        } else if operator == "or" || operator == "||" {
            result = result || self.check_statement(&a, &b, &c,block);
        } else {
            print("error operator on if statement", "r");

            //return false; // Unknown operator or invalid syntax
        }
        index += 4;
    }
        result
    }
    fn math(&mut self,a: &str, method: &str, b: &str,block:&mut NeoCatCodeBlock) -> NeoCatVar {
        // this handles math operations from nscript. this is being looped in nscript_runmath()
        // in case of variables or calls return vallues be used.
        // ----------------------------------------------------------
        let a_val = self.executeword(&a,block).stringdata;
        let b_val = self.executeword(&b, block).stringdata;
        let mut res: f64 = 0.0;

        match method {
            "+" => {
                res = f64(&a_val) + f64(&b_val);
            }
            "-" => {
                res = f64(&a_val) - f64(&b_val);
            }
            "/" => {
                res = f64(&a_val) / f64(&b_val);
            }
            "*" => {
                res = f64(&a_val) * f64(&b_val);
            }
            _ => {
                //
            }
        };
        //println!("math = {a} {b} {c} with result={r}",a = &a_val,b = &method, c = &b_val,r = &res);
        let mut retvar = NeoCatVar::new("math","string");
        retvar.stringdata = res.to_string();
        return retvar;
    }

    fn runmath(&mut self, splitline: &Vec<String>, indexpars: usize, block: &mut NeoCatCodeBlock) -> NeoCatVar {
        // this will perform a line calculation
        // indexpars = where the math begins var = x + 1 mea word[2] is the beginning
        //----------------------------------------

        let mut index = indexpars; // begin after var =
        let mut result = self.math(
            &splitline[index],
            &splitline[index + 1],
            &splitline[index + 2],
            block
        );
        index += 2;
        while index < splitline.len() - 1 {
            result = self.math(&result.stringdata, &splitline[index + 1], &splitline[index + 2],block);
            index += 2;
        }
        return result;
    }
    fn check_statement(&mut self, a: &str, b: &str, c: &str,block:&mut NeoCatCodeBlock) -> bool {
        // this is used to check a single statement in nscript.
        // ---------------------------------------------------------------

        match b {
            "=" | "==" => {
                if &self.executeword(&a,block).stringdata == &self.executeword(&c,block).stringdata {
                    return true;
                }
            }
            "!=" | "<>" => {
                if &self.executeword(&a,block).stringdata != &self.executeword(&c,block).stringdata  {
                    return true;
                }
            }
            ">" => {
                if f64(&self.executeword(&a,block).stringdata) > f64(&self.executeword(&c,block).stringdata) {
                    return true;
                }
            }
            ">=" => {
                if f64(&self.executeword(&a,block).stringdata) >= f64(&self.executeword(&c,block).stringdata) {
                    return true;
                }
            }
            "<=" => {
                if f64(&self.executeword(&a,block).stringdata) <= f64(&self.executeword(&c,block).stringdata) {
                    return true;
                }
            }
            "<" => {
                if f64(&self.executeword(&a,block).stringdata) < f64(&self.executeword(&c,block).stringdata) {
                    return true;
                }
            }
            _ => {
                // error msg will be made.
            }
        }

        return false;
    }
    fn matchscope(&mut self,tomatch:&str,subscope:usize, block:&mut NeoCatCodeBlock) -> NeoCatVar {


        // block.insubblock = line[line.len()-1].parse::<usize>().unwrap_or(0);
        // block.breakloop.push(false);
        // block.breakloop[block.insubblock] = false;
        // let toreturn:Option<NeoCatVar> = None;
        // if let Some(result) = self.executescope(&block.subblockmap[block.insubblock-1].clone(),block){
        //     if result.name == "return"{
        //         return Some(result);
        //     }
        // }
        // return toreturn;
        let matchscope = block.subblockmap[subscope].clone();

        //print("matchscope","g");
        for lines in matchscope{
            if lines.len() >3{
                if lines[0] != "" {
                    if lines[lines.len()-2] == "SCOPE" {
                        for xcheck in 0..lines.len() - 3{
                            if &lines[xcheck] != "|" {
                                let thisvar = self.executeword(&lines[xcheck], block);
                                if &thisvar.stringdata == tomatch && &thisvar.stringdata != ""{
                                    //println!("matched! {}",&lines[xcheck]);
                                    if let Some(mut thisvar) = self.executesubscope(&lines, block){
                                        return thisvar.clone();
                                    }
                                    return NeoCatVar::new("match", "string");
                                }
                                if &lines[xcheck] == "_" {
                                    if let Some(mut thisvar) = self.executesubscope(&lines, block){
                                        return thisvar.clone();
                                    };
                                    return NeoCatVar::new("match", "string");
                                }
                            }
                        }
                    }
                }
            }
        }
        return NeoCatVar::new("error", "string");
    }
}
fn f64(string:&str) ->f64{
    string.parse::<f64>().unwrap_or(0.0)
}

pub fn split<'a>(s: &'a str, p: &str) -> Vec<&'a str> {
    // returns a str array vector
    let r: Vec<&str> = s.split(p).collect();
    return r;
}
fn read_file_utf8(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mut contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut contents) {
        return String::new();
    }

    let (decoded, _, _) = UTF_8.decode(&contents);
    decoded.into_owned()
}

