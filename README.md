# frp
用rust开发的frp

## 开发说明
### Client 端
```bash
cargo run  -p frpc
```
需要传递参数的使用方式 `-h`会传递给`frpc`命令 
```bash
cargo run  -p frpc -- -h
```


### Server 端
```bash
cargo run  -p frps
```
需要传递参数的使用方式 `-h`会传递给`frps`命令 
```bash
cargo run  -p frps -- -h
```
