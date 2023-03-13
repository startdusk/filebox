## 本地开发

### 外部依赖
- PostgresSQL
- Redis

### 如何运行
将 ```env.example``` 修改为 ```.env```, 然后修改为自己对应的变量
```bash
$ mv .env.example .env
```

运行, 推荐使用 `Docker`
```bash
$ make up-dev
```


