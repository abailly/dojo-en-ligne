fn main() {
    println!("Hello, Moon Lander!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy() {
        assert_eq!(2+2, 4);
    }
}

