//构建脚本编译调用c
 
 //1、在Cargo.toml文件中添加  
    //build  = "build.rs"   #构建脚本

    //[dependencies]
    //[build_dependencies] #编译工具的依赖
    //cc = "1.0"
//2、在src目录同级创建需要构建的文件build.rs
    //build.rs功能：先编译C文件成C库
//3、写入内容
//4、在src目录下创建c文件,写入C内容

//在main.rs中使用c


extern { fn hello(); }//导入

fn main() {
    unsafe {//裸指针(不安全)
        hello();
    }
}
