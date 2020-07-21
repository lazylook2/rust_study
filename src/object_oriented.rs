pub fn oo1() {
    pub struct AveragedCollection{
        list: Vec<i32>,
        average: f64
    }
    impl AveragedCollection{
        pub fn add(&mut self, value: i32){
            self.list.push(value);
            self.update_average()
        }
        pub fn update_average(&mut self){
            let sum: i32 = self.list.iter().sum(); // 这里需要明确变量类型
            self.average = sum as f64 / self.list.len() as f64
        }
        pub fn remove(&mut self) -> Option<i32>{
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                },
                None => None
            }
        }
        pub fn average(&self) -> f64 {
            self.average
        }
    }
    let mut c = AveragedCollection{ list: vec![], average: 0.0 };
    c.add(1);
    c.add(2);
    c.add(3);
    println!("平均值：{}", c.average());
    c.remove().unwrap();
    println!("平均值：{}", c.average());
}