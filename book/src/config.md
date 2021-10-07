# 配置


每个项目文件夹下有一个Config.toml

```toml
[server]
port=80  #server default  listening on 80  


[database]
type="mysql"
host="127.0.0.1"
port=3306
database="test"
username="root"
password="root"

[generator]
# compile code to dist directory,default ./dist  
out_dir="./dist" 
# server generate template  ,such as `spring-boot`,netcore `abp`,rust `rocket`
server="spring-boot"
client="ng-alain"

[swagger]
header_file="./swagger/header.yml"
other_files_include="./swagger/*.yml"

```