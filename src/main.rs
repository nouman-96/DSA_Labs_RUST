
#[derive(Debug)]
struct Employee {
    name : String,
    tax_rate : f32
}
impl Employee{
    fn getName(&self){
        println!("Employee Name: {}",self.name);
    }
    fn calcSalary(&self,Sal:f32){
        let salary = Sal * self.tax_rate;
        println!("Employee Salary: {}",salary);

    }
}

#[derive(Debug)]
struct SalariedEmployee {
    name : String,
    salary : i32
}

impl SalariedEmployee{
    fn getName(&self){
        println!("Employee Name: {}",self.name);
    }
    fn calcSalary(&self){
        println!("Employee Salary: {}",self.salary);
    }
}

#[derive(Debug)]
struct HourlyEmployee {
    name : String,
    hours : f64,
    hourly_rate : f64
}

impl HourlyEmployee{
    fn getName(&self){
        println!("Employee Name: {}",self.name);
    }
    fn calcSalary(&self){
        println!("Employee Salary: {}",self.hours * self.hourly_rate);
    }
}

#[derive(Debug)]
struct CommEmployee {
    name : String,
    sales : f32,
    comm_rate : f32
}
impl CommEmployee{
    fn getName(&self){
        println!("Employee Name: {}",self.name);
    }
    fn calcSalary(&self){
        println!("Employee Salary: {}",self.sales * self.comm_rate);
    }
}


fn main() {
    let E1 = Employee {name: "Muneeb".to_string(), tax_rate: 0.4};
    E1.getName();
    E1.calcSalary(50000.0);

    let S1 = SalariedEmployee {name : "Umair".to_string(), salary:50000 };
    S1.getName();
    S1.calcSalary();

    let H1 = HourlyEmployee {name : "Shayan".to_string(), hours : 240.0, hourly_rate: 300.0};
    H1.getName();
    H1.calcSalary();

    let C1 = CommEmployee {name : "Ashahab".to_string(), sales:500.0, comm_rate:0.5};
    C1.getName();
    C1.calcSalary();

}
