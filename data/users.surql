INSERT INTO user [
    {
        name: "abc",
        email: "abc@abc.com",
        password: crypto::argon2::generate("password")
    }
]