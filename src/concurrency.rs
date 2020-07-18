use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};
use std::rc::Rc;

/// 使用 spawn 创建新线程
/// 此程序：执行主线程后就不需要管新线程了
pub fn threads1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }
}
/// 使用 join 等待所有线程结束<br>
/// 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。<br>
/// 此程序：在主线程执行后需要等待handle线程执行完毕（如果handle还没执行完）
pub fn threads2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }
    handle.join().unwrap();
}
/// 使用 join 等待所有线程结束<br>
/// 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。<br>
/// 此程序：在handle线程执行后再执行主线程
pub fn threads3() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }
}
/// 线程与 move 闭包<br>
/// 使用 move 关键字强制获取它使用的值的所有权<br>
pub fn move1() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
/// 使用消息传递在线程间传送数据<br>
/// 使用 mpsc::channel 函数创建一个新的通道；mpsc 是 多个生产者，单个消费者<br>
/// 一旦你熟悉了这项技术，就能使用通道来实现聊天系统，或利用很多线程进行分布式计算并将部分计算结果发送给一个线程进行聚合。
pub fn message_passing(){
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        // 发送端发送信息
        sender.send(val).unwrap();
    });
    let received = receiver.recv().unwrap();
    println!("Got: {}", received);
    // 通道的接收端有两个有用的方法：recv 和 try_recv。
    // recv会[阻塞]主线程执行直到从通道中接收一个值。一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
    // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
    //      如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。
}
/// 通道与所有权转移
pub fn message_passing1(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 变量移动了，就不能输出（所有权移动）
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/// 通道与所有权转移<br>
/// 此程序：每三秒接收一条数据
pub fn message_passing2(){
    let (sender,receiver) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(3));
        }
    });
    // 注意这里有循环
    for received in receiver {
        println!("Got: {}", received);
    }
}
/// 通过克隆发送者来创建多个生产者 <br>
/// let tx1 = mpsc::Sender::clone(&tx); <br>
/// 此程序：每一秒两个发送者都发一条信息
pub fn message_passing3(){
    let (tx, rx) = mpsc::channel();

    // 注意这里
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}
/// 互斥器一次只允许一个线程访问数据<br>
/// 线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。
pub fn mutex(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6
    }
    println!("m = {:?}", m); // 6
}
/// 在线程间共享 Mutex <br>
/// 多线程和多所有权
pub fn mutex1(){
    let counter = Mutex::new(0);
    let counter = Rc::new(Mutex::new(0)); // 使用Rc 和 clone()不安全
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }
    println!("Result: {}", *counter.lock().unwrap());
}

/// 原子引用计数 Arc <br>
/// 原子性类型工作起来类似原始类型，不过可以安全的在线程间共享。
pub fn mutex2(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Arc<T> 和 Rc<T> 有着相同的 API
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}