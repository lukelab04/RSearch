
pub mod CommandParser {

    use std::env;

    #[derive(Debug)]
    pub struct Value {
        pub flag: Option<String>,
        pub val: String,
    }

    pub fn GetInput() -> Vec<Value> {
        let mut input : Vec<Value> = vec![];
        let args : Vec<String> = env::args().collect();
        
        let mut i = 0;
        
        while i < args.len() {
            if i == 0 {i += 1; continue;}

            if args[i].chars().nth(0).expect("Error parsing input string") == '-' {
                if i == args.len() - 1 {
                    input.push(Value{flag: Some(String::from(&args[i])), val: String::from("")});
                    i += 1;
                    continue;
                }
                let flag = &args[i];
                i+=1;
                if args[i].chars().nth(0).expect("Error parsing input") == '-' {continue;}
                input.push(Value { flag: Some(String::from(flag)), val: String::from(&args[i]) });
            } else {input.push(Value { flag: None, val: String::from(&args[i]) });}

            i += 1;
        }

        return input;
    }
}