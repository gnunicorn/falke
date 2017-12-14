
extern crate tiny_keccak; // brings SHAKE algorithm
extern crate rand;        // System Random Number Generator 

#[cfg(test)]
extern crate hex;


mod shake;


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
