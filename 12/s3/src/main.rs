#[derive(Debug, Default)]
struct Employee {
    name: String,
    position: String,
    salary: f64,
    years_of_service: u32,
}

struct EmployeeBuilder {
    name: String,
    position: Option<String>,
    salary: Option<f64>,
    years_of_service: Option<u32>,
}

impl EmployeeBuilder {
    fn new(name: String) -> Self {
        EmployeeBuilder {
            name,
            position: None,
            salary: None,
            years_of_service: None,
        }
    }

    fn position(&mut self, position: String) -> &mut Self {
        self.position = Some(position);
        self
    }

    fn salary(&mut self, salary: f64) -> &mut Self {
        self.salary = Some(salary);
        self
    }

    fn years_of_service(&mut self, years: u32) -> &mut Self {
        self.years_of_service = Some(years);
        self
    }

    fn build(&self) -> Employee {
        Employee {
            name: self.name.clone(),
            position: self.position.clone().unwrap_or_else(|| "Unknown".to_string()),
            salary: self.salary.unwrap_or(0.0),
            years_of_service: self.years_of_service.unwrap_or(0),
        }
    }
}

fn main() {
    let new_employee = EmployeeBuilder::new("John".to_string())
        .position("Manager".to_string())
        .salary(50000.0)
        .years_of_service(5)
        .build();

    println!("{:?}", new_employee);

    let entry_level_employee = EmployeeBuilder::new("Alice".to_string())
        .build();

    println!("{:?}", entry_level_employee);
}