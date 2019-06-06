use memchr::memchr;


struct SimpleIt<'b> {
    s: &'b str,
    step: usize
}


impl<'b> SimpleIt<'b> {
    fn new(s: &'b str) -> Self {
        SimpleIt {
            s,
            step: 0,
        }
    }
}


impl<'b> Iterator for SimpleIt<'b> {
    type Item = &'b str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s.len() > self.step {
            let end = self.step + 1;
            let item = &self.s[self.step .. end];
            self.step = end;
            return Some(item)
        }
        None
    }
}


struct KeyValue<'a> {
    s: &'a str,
    spt_pair: u8,
    spt_kv: u8,
}


impl<'a> KeyValue<'a> { 
    fn new(s: &'a str, spt_pair: u8, spt_kv: u8) -> Self {
        KeyValue {
            s,
            spt_pair,
            spt_kv,
        }
    }
}



impl<'a> Iterator for KeyValue<'a> {
    type Item = (&'a str, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let key: &'a str;
        let value: &'a str;
        let end = self.s.len();
        if end > 1 {
            match memchr(self.spt_kv, &self.s.as_bytes()) {
                Some(end_key) => {
                    key = &self.s[.. end_key];
                    let seporator = match memchr(self.spt_pair, &self.s[end_key + 1 ..].as_bytes()) {
                        Some(sp) => sp + end_key + 1,
                        _ => end,
                    };
                    let droper = match memchr(b'"', &self.s[end_key + 1 ..].as_bytes()) {
                        Some(dr) => dr + end_key + 1,
                        _ => seporator + 1,
                    };
                    if droper < seporator {
                        let end_droper = match memchr(b'"', &self.s[droper + 1 ..].as_bytes()) {
                            Some(v) => v + droper + 1,
                            _ => end,
                        };
                        value = &self.s[droper + 1 .. end_droper];
                        if end <= end_droper + 1 {
                            self.s = "";
                        } else {
                            self.s = &self.s[end_droper + 2 ..];
                        }
                    } else {
                        value = &self.s[end_key + 1 .. seporator];
                        if seporator < end {
                            self.s = &self.s[seporator + 1 ..];
                        } else {
                            self.s = "";
                        }
                    }
                }
                _ => return None,
            }
            let item = (key, value);
            return Some(item);
        } 
        None
    }
}


pub fn main() {
    for (key, value) in KeyValue::new("k1=v1,k2=v2,k3=v3,k4=v4", b',' , b'=') {
        println!("{} - {}", key, value);
    }
    println!("-------------------------------------");

    for (key, value) in KeyValue::new("key=value,key2=v,k3=\"v30000,v4\"", b',' , b'=') {
        println!("{} - {}", key, value);
    }
    println!("-------------------------------------");

    for (key, value) in KeyValue::new("key=value,key2=v,k3=\"v300<<lll,,,,00,ll\",k4=v4,k5=v5", b',' , b'=') {
        println!("{} - {}", key, value);
    }
    println!("-------------------------------------");

    for (key, value) in KeyValue::new("key=value,key2=\"v\",k3=\"v300<<lll,,,,00,v4\",k4=\"\",k5=\",\",   k6=v6,k7=\"v7\",k8=v8", b',' , b'=') {
        println!("{} - {}", key, value);
    }
    println!("-------------------------------------");

    for i in SimpleIt::new("Hello, world!") {
        println!("{}", i);
    }
}