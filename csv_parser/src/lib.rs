
pub struct CsvHelper<'a> {
    reader: &'a mut ::std::io::BufRead
}

impl<'a> CsvHelper<'a> {

    fn new(reader: &'a mut ::std::io::BufRead) -> CsvHelper<'a> {
        CsvHelper {
            reader: reader
        }
    }
}

pub struct CsvRecords<T: Iterator> {
    records: T
}

macro_rules! model {
    ($name:ident {
        $($fname:ident: $ftype:ty,)+
    }) => {
        #[derive(Debug)]
        pub struct $name {
            $($fname: $ftype,)+
        }

        impl $crate::std::str::FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<$name, Self::Err> {
                let mut iter = s.split(",");
                Ok($name {
                    $(
                        $fname: iter.next()
                            .expect(stringify!($fname))
                            .parse::<$ftype>()
                            .unwrap(),
                    )+
                })
            }
        }

        impl<'a> IntoIterator for &'a mut CsvHelper<'a> {
            type Item = $name;
            type IntoIter = CsvRecords<$crate::std::io::Lines<&'a mut $crate::std::io::BufRead>>;

            fn into_iter(self) -> Self::IntoIter {
                use $crate::std::io::BufRead;

                CsvRecords { records: self.reader.lines() }
            }
        }

        impl<R: $crate::std::io::BufRead> Iterator for CsvRecords<$crate::std::io::Lines<R>>  {
            type Item = $name;

            fn next(&mut self) -> Option<Self::Item> {
                match self.records.next() {
                    Some(x) => match x {
                        Ok(s) => s.parse().ok(),
                        Err(_) => None
                    },
                    None => None
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    model! {
        Test {
            a: i32,
            b: i32,
            c: u32,
        }
    }

    #[test]
    fn test_read_from_file() {
        let mut reader = ::std::io::BufReader::new(::std::fs::File::open("test.csv").unwrap());
        let mut csv_reader = CsvHelper::new(&mut reader);

        for item in &mut csv_reader {
            println!("{:?}", item);
        }

        assert!(false);
    }

}
