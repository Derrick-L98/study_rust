use console::{style, Term};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub async fn console() {
    for i in 0..=255 {
        print!("{:03} ", style(i).color256(i));
        if i % 16 == 15 {
            println!("");
        }
    }

    for i in 0..=255 {
        // 底色: 黑色
        // 上边: 对应色
        print!("{:03} ", style(i).black().on_color256(i));
        if i % 16 == 15 {
            println!("");
        }
    }

    println!("{}", style("hello").color256(196));

    for i  in 1..=9 {
        for x in 1..=i {
            print!("{} x {} = {:>2} ", style(x).color256(x), style(i).color256(x), style(i*x).color256(x))
        }
        println!("");
    }
}

pub async fn write_chars() -> io::Result<()> {
    let term = Term::stdout(); //返回新的无缓冲端子
    let (heigth, width) = term.size(); //返回行和列中的终端大小或获取合理的默认值。
    for x in 0..width {
        for y in 0..heigth {
            term.move_cursor_to(x as usize, y as usize)?; //将光标移动到x和y
            let text = if (x + y) % 2 == 0 {
                format!(
                    "{}",
                    style(x % 10) //包装对象以设置样式的格式。
                        // .green()
                        .black()
                        // .on_green()
                        .on_red()
                )
            } else {
                format!(
                    "{}",
                    style(x % 10) //包装对象以设置样式的格式。
                        .red()
                        .on_black()
                )
            };

            term.write_str(&text)?;
            thread::sleep(Duration::from_micros(600));
        }
    }
    Ok(())
}

pub async fn do_stuff() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("Counting...");
    term.write_line("Going to do some counting now")?;
    term.hide_cursor()?;
    for x in 0..10 {
        if x != 0 {
            term.move_cursor_up(1)?;
        }
        term.write_line(&format!("Counting {}/10", style(x + 1).cyan()))?;
        thread::sleep(Duration::from_millis(400));
    }
    term.show_cursor()?;
    term.clear_last_lines(1)?;
    term.write_line("Done counting!")?;
    writeln!(&term, "Hello World!")?;

    write!(&term, "To edit: ")?;
    let res = term.read_line_initial_text("default")?;
    writeln!(&term, "\n{}", res)?;

    Ok(())
}

pub async fn test() {
    println!(
        "This is red on black: {:010x}",
        style(42).red().on_black().bold()
    );
    println!("This is reversed: [{}]", style("whatever").reverse());
    println!("This is cyan: {}", style("whatever").cyan());
    eprintln!(
        "This is black bright: {}",
        style("whatever").for_stderr().bright().black()
    );
}

pub async fn print_da() {
    let mut a: f32 = 0.0;
    let mut x: f32 = -1.5;
    let mut y: f32 = 1.5;
    // for(y=1.5f; y>-1.5f; y-=0.1f)
    // {
    // 	for(x=-1.5f; x<1.5f; x+=0.05f)
    // 	{
    // 		a = x*x+y*y-1;
    // 		//这里的@符号即为打印出的心形图案符号，可更改
    // 		char ch = a*a*a-x*x*y*y*y<=0.0f?'@':' ';
    // 		putchar(ch);
    // 		//或者putchar(a*a*a-x*x*y*y*y<=0.0f?'*':' ');
    // 	}
    // 	printf("\n");
    // }

    loop {
        if y > -1.5 {
            println!("y = {}", y);
            if x < 1.5 {
                println!("x = {}", x);
                a = x * x + y * y - 1.0;
                println!("a = {}", a);
                println!(" t = {}", a * a * a - x * x * y * y * y);
                if (a * a * a - x * x * y * y * y) <= 0.0 {
                    println!("1{}", '@');
                } else {
                    println!("2{}", ' ');
                }
                x += 0.05;
            }
            y -= 0.1;
        } else {
            return;
        }
    }
}
