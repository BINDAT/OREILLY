#[test] // Test la fonction gcd
fn test_gcd() { // Test de quelques cas simples
    assert_eq!(gcd(14, 15), 1); // facteurs premiers de 14: 2, 7; facteurs premiers de 15: 3, 5

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                3 * 7 * 11 * 13 * 19),
            3 * 11); // facteurs premiers de 2*3*5*11*17: 2, 3, 5, 11, 17; facteurs premiers de 3*7*11*13*19: 3, 7, 11, 13, 19
}