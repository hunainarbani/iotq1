pub mod vehicle_main {

    #[derive(Debug)]
    pub struct Vehicle{
        Name: String,
        Model: String,
        Capacity: i32,
        Color:String,
        Transmission:String
    }

    pub fn input_vehicle(vName: String,vModel:String,vCapacity:i32,
                             vColor:String,vTransmission:String) -> Vehicle
    {            
            let vech_item =          
            Vehicle {
                Name: vName,
                Model: vModel,
                Capacity: vCapacity,
                Color: vColor,
                Transmission: vTransmission
            };

            return vech_item
    }

    
    impl Vehicle {

         pub fn Vehicle_Detail(&self) -> String{
             format!("Vehicle Info\nName : {}\nModel : {}\nCapcity : {}\nColor : {}\nTranmission : {}"
             ,self.Name,self.Model,self.Capacity,self.Color,self.Transmission)

         }
    }
}