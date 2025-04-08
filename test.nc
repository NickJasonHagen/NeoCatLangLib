


thread thisthread {
   func threadreceive(recv){
     if recv != "" {
        if 1 == 1 {
             ret = cat "hello " recv
        }
        ret
     }
   }

   coroutine "mainthread" {
    //er moet tenminste 1 loops zijn anders exit
   }

}
spawnthread oi thisthread
coroutine "main" {
    bullshit = "agagagag"

        //oi = "oi"
        something = threadsend::oi(bullshit)

        if something != ""{
            b = ""
            b &= "m: " something
            //b !!
        }
}

$cmdarg0 !!
$cmdarg1 !!
$cmdarg2 !
