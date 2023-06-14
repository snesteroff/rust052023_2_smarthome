#[derive(PartialEq, Debug, Clone, Copy)]
enum OnOff {
    On,
    Off,
}

struct ElectricalOutlet {
    on_off: OnOff,
    description: String,
    current_power: f32,
    switched_power: f32,
}
struct Thermometer {
    temperature: f32,
}

impl ElectricalOutlet {
    fn new(target: String) -> Self {
        ElectricalOutlet {
            on_off: OnOff::Off,
            description: target,
            current_power: 0.0,
            switched_power: 50.0,
        }
    }
    fn get_description(&self) -> &String {
        &self.description
    }
    fn switch(&mut self, onf: OnOff) -> OnOff {
        if self.on_off != onf {
            self.on_off = onf;
        }
        if self.on_off == OnOff::On {
            self.current_power = self.switched_power;
        } else {
            self.current_power = 0.0;
        }
        self.on_off
    }
    fn get_power(&self) -> f32 {
        self.current_power
    }
}

impl Thermometer {
    fn new() -> Self {
        Thermometer { temperature: 20.0 }
    }
    fn read_temperature(&self) -> f32 {
        self.temperature
    }
}

fn main() {
    let door = String::from("Под дверью");
    let mut elot = ElectricalOutlet::new(door);
    let therm = Thermometer::new();
    println!("Temepretuare is {} degrees", therm.read_temperature());
    println!("Desription of the EO \"{}\"", elot.get_description());
    let on = elot.switch(OnOff::On);
    println!("Now is swicthed {:?}!", on);
    let power = elot.get_power();
    println!("Current power is {}", power);

    let off = elot.switch(OnOff::Off);
    println!("Now is swicthed {:?}!", off);
    let power = elot.get_power();
    println!("And now current power is {}", power);
}
