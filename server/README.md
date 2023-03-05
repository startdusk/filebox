# TODO: 待添加文档

## 使用方式

### 外部依赖
- PostgresSQL
- Redis

### 如何运行
将 ```env.example``` 修改为 ```.env```, 然后修改为自己对应的变量
```bash
$ mv .env.example .env
```

```bash
$ make run
```

## 遇到的问题集合

### error: linking with `cc` failed: exit status: 1
算是个官方的BUG吧
解决方式
```bash
rustup toolchain uninstall stable && rustup toolchain install stable
```

