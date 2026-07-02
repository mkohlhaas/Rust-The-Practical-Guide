...
impl Car {
	fn display_car_info(&self) { // car: &Car changed to &self
		println!(
		“owner: {}, Year: {}, Price: {}”, 
		self.owner, self.year, self.price);    
	}
}
...
}
