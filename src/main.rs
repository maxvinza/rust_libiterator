use memchr::memchr;


struct KeyValue {
    s: String,
    start: usize,
    spt_pair: u8,
    spt_kv: u8,
}


impl KeyValue { 
    fn new(s: &str, spt_pair: u8, spt_kv: u8) -> Self {
        KeyValue {
            s: s.to_string(),
            start: 0,
            spt_pair: spt_pair,
            spt_kv: spt_kv,
        }
    }
}


struct SimpleIt {
    s: String,
}


impl SimpleIt {
    fn new(s: &str) -> Self {
        SimpleIt {
            s: s.to_string(),
        }
    }
}


impl Iterator for SimpleIt {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s.len() > 0 {
            let item = self.s.remove(0);
            return Some(item)
        }
        None
    }
}


impl<'a> Iterator for  &'a KeyValue {
    type Item = (&'a str, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut key: &'a str;
        let mut value: &'a str;
        let sb: &'a str = &self.s.as_str()[self.start ..];
        let end = sb.len();
        if end > 1 {
            match memchr(self.spt_kv, sb.as_bytes()) {
                Some(end_key) => {
                    key = &sb[.. end_key];
                    let seporator = match memchr(self.spt_pair, &sb[end_key + 1 ..].as_bytes()) {
                        Some(sp) => sp,
                        _ => end,
                    };
                    let droper = match memchr(b'"', &sb[end_key + 1 ..].as_bytes()) {
                        Some(dr) => dr,
                        _ => seporator + 1,
                    };
                    if droper < seporator {
                        value = match memchr(b'"', &sb[droper + 1 ..].as_bytes()){
                            Some(end_droper) => &sb[droper + 1 .. end_droper],
                            _ =>  &sb[droper + 1 .. end],
                        }
                    } else {
                        value = &sb[end_key +1 .. seporator];
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
    let mut map =  KeyValue::new("key=value,key2=v,k3=\"v3,v4\"", b',' , b'=');
    let mut i = 0;
    for (key, value) in &map {
        i += 1;
        if i > 10 {
            break;
        }
        println!("{} - {}", key, value);
    }

    for i in SimpleIt::new("Hello, world!") {
        println!("{}", i);
    }
}