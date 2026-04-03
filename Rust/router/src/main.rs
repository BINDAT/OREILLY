fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println("Serveur en http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}