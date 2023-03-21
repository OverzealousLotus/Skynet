// Rust version of my Terminator Tycoon.

use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn intake(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.")
        .to_string();

    input // stdin() leaves whitespace at end of String. Use trim() to remove.
}

fn wait(time: f64) {
    sleep(Duration::from_secs_f64(time));
}

struct T800s {
    units: usize,
    _power: usize,
}
struct Components {
    titanium: usize,
    _coltan: usize,
}
struct Logistics {
    cores: usize,
    recycler: usize,
}
struct Costs {
    core_cost: usize,
    recycle_cost: usize,
}

struct GameResources {
    components: Components,
    modules: Logistics,
    costs: Costs,
    t800: T800s,
}

const T800_COST: usize = 5;

impl GameResources {
    fn main_menu(&mut self) {
        // Main Menu for the Program.

        'mainloop: loop {
            println!(
                "
                \n<===| Available Commands |===>,
                \n|=====> Produce,
                \n|=====> Evaluate,
                \n|=====> Assemble,
                \n|=====> Attack,
                \n|=====> Adjust"
            );
            let decision: String = intake("Execute command: ");

            match decision.to_lowercase().as_str().trim() {
                "produce" => {
                    self.produce();
                }

                "evaluate" | "eval" => {
                    self.evaluate();
                }

                "assemble" => {
                    self.assemble();
                }

                "attack" => {
                    self.attack();
                }

                "adjust" => {
                    self.adjust();
                }

                "quit" | "exit" | "shutdown" => {
                    println!("Powering off Central Core...");
                    break 'mainloop;
                }

                _ => println!("Command not recognized. Aborting!"),
            }
        }
    }

    fn produce(&mut self) {
        // Produce Titanium
        if let Ok(mut countdown) = intake("Production time: ").trim().parse::<usize>() {
            format!("Beginning production for {countdown} seconds.");
            while countdown != 0 {
                countdown -= 1;
                self.components.titanium += self.modules.recycler;
                wait(0.8);
                println!(
                    "
                    Procured 1KG of titanium. New sum: {}",
                    &self.components.titanium
                );
            }
        } else {
            println!("Unrecognized timeframe used. Aborting!");
        }
    }

    fn evaluate(&self) {
        println!(
            "
            \n<===== Statistics =====>
            \n|===> Titanium Reserves: {}
            \n|===> Core Count: {}
            \n|===> Recycler Eff. {}
            \n<=== Forces ===>
            \n|===> T-800 Units: {}",
            &self.components.titanium,
            &self.modules.cores,
            &self.modules.recycler,
            &self.t800.units
        );
    }

    fn assemble(&mut self) {
        println!(
            "
            \n<===== Models =====>,
            \n|===> T-800 | Cost: {}",
            &self.t800.units
        );

        let model: String = intake("Type: ");

        if self.components.titanium < T800_COST {
            println!("Insufficient Titanium reserves. Aborting!");
        }

        if model.to_lowercase().trim() == "t800" {
            let count = intake("Count: ").trim().parse::<usize>();
            if let Ok(units) = count {
                self.components.titanium -= T800_COST * units;
                self.t800.units += units;
                println!("Produced {} T-800s!", &units);
            } else {
                println!("Invalid count, aborting!");
            }
        } else {
            println!("Invalid Model entered... Aborting!");
        }
    }

    fn attack(&mut self) {
        println!("In development...");
    }

    fn adjust(&mut self) {
        println!(
            "
            \n<===== Systems =====>
            \n|===> Cores: {}
            \n|===> Recycler Eff. {}",
            &self.modules.cores, &self.modules.recycler
        );
        let upgrade = intake("Adjustment type: ");

        match upgrade.trim().to_lowercase().as_str() {
            "cores" if self.components.titanium >= self.costs.core_cost => {
                self.components.titanium -= self.costs.core_cost;
                self.modules.cores += 1;
                self.costs.core_cost *= 2;
                println!("Core successfully upgraded!");
            }

            "recycler" if self.components.titanium >= self.costs.recycle_cost => {
                self.components.titanium -= self.costs.recycle_cost;
                self.modules.recycler += 1;
                self.costs.recycle_cost *= 2;
                println!("Recycler adjustment successful!");
            }

            _ => {
                println!("Invalid system component, or insufficient Titanium. Aborting!");
            }
        }
    }
}

fn main() {
    println!("\x1b[0;30;41m Booting up Central Core...");
    wait(1.0);
    println!("Done! Initalizing Primary Directives...");
    wait(0.75);
    println!("Done! Contacting External Factories...");
    wait(2.0);
    println!("Complete! \n\n\n");
    let mut resources: GameResources = GameResources {
        components: Components {
            titanium: 0,
            _coltan: 0,
        },
        modules: Logistics {
            cores: 1,
            recycler: 1,
        },
        costs: Costs {
            core_cost: 15,
            recycle_cost: 15,
        },
        t800: T800s {
            units: 0,
            _power: 10,
        },
    };
    resources.main_menu();
}
