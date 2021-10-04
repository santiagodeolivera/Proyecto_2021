mod parser;

/*
from_input! {

    #[toLowercase]
    bool: multipleOption {
        "yes": "y" => true,
        "no": "n" => false
    }


    LogInData: form {
        data: {
            name: TrimmedStr,
            password: TrimmedStr
        },
        func: LogInData::new(name, password)
    }

    TrimmedStr: simple {
        func(s) => TrimmedStr::new(s)
    }
}
*/


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
