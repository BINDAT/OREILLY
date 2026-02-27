fn gcd(mut n: u64, mut m: u64) -> u64 { // La fonction prend deux nombres entiers non signés de 64 bits en entrée et retourne leur plus grand commun diviseur
    assert!(n != 0 && m != 0); // Le plus grand commun diviseur n'est pas défini pour zéro, donc on vérifie que les deux nombres sont différents de zéro
    while m != 0 { // Tant que m n'est pas égal à zéro, on continue à calculer le plus grand commun diviseur
        if m < n { // Si m est plus petit que n, on échange les deux nombres pour que m soit toujours le plus grand
            let t = m; // On stocke m dans une variable temporaire t
            m = n; // On échange m et n pour que m soit toujours le plus grand des deux nombres
            n = t; // On échange n et m pour que n soit toujours le plus petit des deux nombres
        }
        m = m % n; // Le reste de la division de m par n, qui est plus petit que n, devient le nouveau m
    }
    n // Le plus grand commun diviseur
}