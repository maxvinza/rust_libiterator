use memchr::memchr;


struct KeyValue {
    s: String,
    start: usize,
    spt_pair: char,
    spt_kv: char,
}


impl KeyValue { 
    fn new(s: &str, spt_pair: char, spt_kv: char) -> Self {
        KeyValue {
            s: s.to_string(),
            start: 0,
            spt_pair,
            spt_kv,
        }
    }
}


impl<'a> Iterator for  &'a KeyValue {
    type Item = (&'a str, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut key: &'static str = "";
        let mut value: &'static str = "";
        key = "test";
        /*
        let sb = &self.s[self.start ..];
        let end = sb.len();
        if end > 1 && sb < end {
            match memchr(self.spt_kv, sb) {
                Some(end_key) => {
                    key = sb[.. end_key];
                    let seporator = match memchr(self.spt_pair, sb[end_key + 1 ..]) {
                        Some(sp) => sp,
                        _ => end,
                    };
                    let droper = match memchr(b'"', sb[end_key + 1 ..]) {
                        Some(dr) => dr,
                        _ => seporator + 1,
                    };
                    if droper < seporator {
                        value = match memchr(b'"', sb[droper + 1 ..]){
                            Some(end_droper) => sb[droper + 1 .. end_droper],
                            _ =>  sb[droper + 1 .. end],
                        }
                    } else {
                        value = sb[end_key +1 .. seporator];
                    }
                }
                _ => return None,
            }
            let item = (key, value);
            return Some(item);
        } */
        let item = (key, value);
        return Some(item);
    }
}


pub fn main() {
    let mut map =  KeyValue::new("key=value,key2=v,k3=\"v3,v4\"", ',' , '=');
    let mut i = 0;
    for (key, value) in &map {
        i += 1;
        if i > 10 {
            break;
        }
        println!("{} - {}", key, value);
    }
}