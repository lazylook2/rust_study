pub fn oo1() {
    pub struct AveragedCollection{
        list: Vec<i32>,
        average: f64
    }
    impl AveragedCollection{
        pub fn add(&mut self, value: i32){
            self.list.push(value);
        }
        pub fn update_average(&self) -> f64{
            let sum = self.list.iter().sum();
            sum / self.list.len()
        }
        pub fn remove(&mut self) {
            let result = self.list.pop();
            match result {
                Some(value) => {},
                None => {}
            }
            self.update_average();
        }
    }
}