impl TravelType {
    fn travel_allowance(&self) -> f32 { // parameter miles: f32 is no longer required
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0, 
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Airplane(miles) => miles * 5.0,
        }
	allowance
    }
}