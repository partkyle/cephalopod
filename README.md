# cephalopod

Example file upload using actix-web.

## usage

```
~/c/r/cephalopod (master) > cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.62s
     Running `target/debug/cephalopod`
Starting http server: 127.0.0.1:8080
[2018-12-17T18:40:24Z INFO  actix_web::middleware::logger] 127.0.0.1:58748 "POST / HTTP/1.1" 200 280 "-" "curl/7.54.0" 51.875499
```

Then upload a file

```
~/c/r/cephalopod (master) > time curl localhost:8080 -F upload=@target/debug/cephalopod
[{"content_type":"application/octet-stream","sha":"ad635e873c9dcea595685e87255c38494244db69"}]

4.95 real         0.01 user         0.03 sys
```
