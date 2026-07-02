Impl Car {
	fn display_car_info(&self) { // First form: &self
		println!(
		“owner: {}, Year: {}, Price: {}”, 
		self.owner, self.year, self.price);    
	}
    	fn refuel(&mut self, gallons: f32) { // Second form: &mut self
        	self.fuel_level += gallons;
    	}
   	fn sell(self) -> Self { // Third form: self
        	self
    	}
}