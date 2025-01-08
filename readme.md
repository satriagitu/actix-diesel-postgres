### How to use Diesel

Tutorial: https://diesel.rs/guides/getting-started

Run Diesel
```
diesel migration run
```

### How to Run Application
```
cargo run
```


CURL
```
curl -X POST http://127.0.0.1:8080/users -H "Content-Type: application/json" -d '{
    "name": "John Doe",
    "email": "johndoe@example.com"
}'

curl http://127.0.0.1:8080/users/{uid}
```