

#[derive(Debug)]
pub enum Access {
    Owner,
    Admin,
    Moderator,
    Player
}

/// Specifies the value of money types in the lowest money type
pub struct Conversion {
    copper: i32,
    bronze: i32,
    silver: i32,
    gold: i32
}

impl Conversion {
    pub fn new() -> Self {
        Self { copper: 1, bronze: 10, silver: 100, gold: 1000 }
    }

}

#[derive(Debug)]
pub struct Purse {
    copper: Money,
    bronze: Money,
    silver: Money,
    gold: Money
}

impl Purse {
    pub fn new() -> Self {
        Self { copper: Money::Copper(0), bronze: Money::Bronze(0), silver: Money::Silver(0), gold: Money::Gold(0) }
    }

    pub fn receive_money(&mut self, money: Money) -> &mut Self {
        
        match money {
            Money::Copper(n) => {
                let amt = match self.copper {
                    Money::Copper(amt) => amt,
                    _ => 0,
                };
                self.copper = Money::Copper(amt + n);
            },
            Money::Bronze(n) => {
                let amt = match self.copper {
                    Money::Bronze(amt) => amt,
                    _ => 0,
                };
                self.bronze = Money::Bronze(amt + n);
            },
            Money::Silver(n) => {
                let amt = match self.copper {
                    Money::Silver(amt) => amt,
                    _ => 0,
                };
                self.silver = Money::Silver(amt + n);
            }
            Money::Gold(n) => {
                let amt = match self.copper {
                    Money::Gold(amt) => amt,
                    _ => 0,
                };
                self.gold = Money::Gold(amt + n);
            },
        };
        self
    }

    pub fn spend_money(&mut self, money: Money) -> &mut Self {
        
        match money {
            Money::Copper(n) => {
                let amt = match self.copper {
                    Money::Copper(amt) => amt,
                    _ => 0,
                };
                self.copper = Money::Copper(amt - n);
            },
            Money::Bronze(n) => {
                let amt = match self.copper {
                    Money::Bronze(amt) => amt,
                    _ => 0,
                };
                self.bronze = Money::Bronze(amt - n);
            },
            Money::Silver(n) => {
                let amt = match self.copper {
                    Money::Silver(amt) => amt,
                    _ => 0,
                };
                self.silver = Money::Silver(amt - n);
            }
            Money::Gold(n) => {
                let amt = match self.copper {
                    Money::Gold(amt) => amt,
                    _ => 0,
                };
                self.gold = Money::Gold(amt - n);
            },
        };
        self
    }

    pub fn normalize(&mut self, conversion: &Conversion) -> &mut Self {
        let mut total = self.copper.to_copper(conversion)
            + self.bronze.to_copper(conversion)
            + self.silver.to_copper(conversion)
            + self.gold.to_copper(conversion);

            let gold = total / conversion.gold;
            total = total - gold * conversion.gold;
            let silver = total / conversion.silver;
            total = total - silver * conversion.silver;
            let bronze = total / conversion.bronze;
            total = total - bronze * conversion.bronze;

            self.copper = Money::Copper(total); 
            self.bronze = Money::Bronze(bronze);
            self.silver = Money::Silver(silver);
            self.gold = Money::Gold(gold);

        self
    }



}

#[derive(Debug)]
pub enum Money {
    Copper(i32),
    Bronze(i32),
    Silver(i32),
    Gold(i32),
}

impl Money {
    pub fn to_copper(&self, conversion: &Conversion) -> i32 {
        let amt = match self {
            Money::Copper(n) => *n,
            Money::Bronze(n) => *n * conversion.bronze,
            Money::Silver(n) => *n * conversion.silver,
            Money::Gold(n) => *n * conversion.gold
        };
        amt
    }
}

#[derive(Debug)]
pub enum Items {
    Chest,
    Sword,
    Dagger,
    Key,
    Money(Money),
}

#[derive(Debug)]
pub struct Identity {
    name: String,
    access: Access,
    inventory: Vec<Items>,
}

impl Identity {
    pub fn new(name: String, access: Access) -> Self {
        Self { name, access, inventory: vec![] }
    }

    pub fn addItem(&mut self, item: Items) -> &mut Self {
        self.inventory.push(item);
        self
    }
}


#[derive(Debug)]
pub enum Exits {
    North(Room),
    South(Room),
    East(Room),
    West(Room)
}

#[derive(Debug)]
pub struct Room {
    id: u64,
    name: String,
    description: String,
    items: Vec<Items>,
    exits: Vec<Exits>

}

#[cfg(test)] 
mod tests {
    use super::*;


    #[test]
    fn purse_accepts_money() {
        let mut p = Purse::new();
        p.receive_money(Money::Gold(1));
        match p.gold {
            Money::Gold(n) => assert_eq!(n, 1),
            _ => panic!("Gold field contains other Money type")
        }
    }

    #[test]
    fn purse_normalizes() {
        let mut p = Purse::new();
        let conv = Conversion::new();
        p.receive_money(Money::Copper(2143));
        p.normalize(&conv);
        match (p.copper, p.bronze, p.silver, p.gold) {
            (
                Money::Copper(c),
                Money::Bronze(b),
                Money::Silver(s),
                Money::Gold(g) 
            ) => {
                assert_eq!(3, c, "Copper {}", c);
                assert_eq!(4, b, "Bronze {}", b);
                assert_eq!(1, s, "Silver {}", s);
                assert_eq!(2, g, "Gold   {}", g);
            },
            (_,_,_,_) => panic!("Mismatched types"),

        }
    }        

}
