use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn increase_by_one(item: &mut Item) {
        if item.quality < 50 {
            item.quality = item.quality + 1
        }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            
            match item.name.as_str() {
               "Sulfuras, Hand of Ragnaros" => 
                (),
               "Aged Brie" =>
                {
                    GildedRose::increase_by_one(item);
                    if item.sell_in <= 0 {
                       GildedRose::increase_by_one(item);
                    }
                },
               "Backstage passes to a TAFKAL80ETC concert" =>
               {
                    if item.sell_in > 0 {
                       GildedRose::increase_by_one(item);
                        if item.sell_in < 6 {
                           GildedRose::increase_by_one(item);
                        }
                        if item.sell_in < 11 {
                           GildedRose::increase_by_one(item);
                        }
                    } else {
                        item.quality = 0
                    }
               },                                  
               _ => {
                    if item.sell_in > 0 {
                        item.quality = item.quality - 1
                    } else {
                        item.quality = 0
                    }
               }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};
    
    pub fn days_loop(rose:&mut GildedRose,mut days:i32) {
                      
        while days > 0 {
            rose.update_quality();
            days -= 1;
        }

    }

   #[test]
   pub fn default_item() {
        pub struct Test {
            pub days: i32,
            pub output: Vec<Item>,
        }           

        let tests = vec![
            Test {days: 3, output: vec![
                Item::new("+5 Dexterity Vest", 7, 17),
                Item::new("Aged Brie", -1, 4),
                Item::new("Elixir of the Mongoose", 2, 4),
                Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
                Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 12, 23),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 7, 50),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 2, 50),
                Item::new("Conjured Mana Cake", 0, 3),
            ]},
            Test {days: 10, output: vec![
                Item::new("+5 Dexterity Vest", 0, 10),
                Item::new("Aged Brie", -8, 18),
                Item::new("Elixir of the Mongoose", -5, 0),
                Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
                Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 35),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 50),
                Item::new("Backstage passes to a TAFKAL80ETC concert", -5, 0),
                Item::new("Conjured Mana Cake", -7, 0),
            ]},
            Test {days: 29, output: vec![
                Item::new("+5 Dexterity Vest", -19, 0),
                Item::new("Aged Brie", -27, 50),
                Item::new("Elixir of the Mongoose", -24, 0),
                Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
                Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
                Item::new("Backstage passes to a TAFKAL80ETC concert", -14, 0),
                Item::new("Backstage passes to a TAFKAL80ETC concert", -19, 0),
                Item::new("Backstage passes to a TAFKAL80ETC concert", -24, 0),
                Item::new("Conjured Mana Cake", -26, 0),
            ]},
        ];

        for test in tests{

            let input = vec![
                Item::new("+5 Dexterity Vest", 10, 20),
                Item::new("Aged Brie", 2, 0),
                Item::new("Elixir of the Mongoose", 5, 7),
                Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
                Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49),
                Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49),
                Item::new("Conjured Mana Cake", 3, 6),
            ];

            let mut rose = GildedRose::new(input);
            days_loop(&mut rose,test.days);
            for i in 0..test.output.len(){
                assert_eq!(
                    (test.output[i].sell_in,test.output[i].quality),
                    (rose.items[i].sell_in,rose.items[i].quality)
                );
            }
        }
   }

}
