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

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            } else {
                if item.quality < 50 {
                    item.quality = item.quality + 1;

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.quality = item.quality - 1;
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
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

        let output = vec![
            Item::new("+5 Dexterity Vest", -19, 0),
            Item::new("Aged Brie", -27, 50),
            Item::new("Elixir of the Mongoose", -24, 0),
            Item::new("Sulfuras, Hand of Ragnaros", -0, 80),
            Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
            Item::new("Backstage passes to a TAFKAL80ETC concert", -14, 0),
            Item::new("Backstage passes to a TAFKAL80ETC concert", -19, 0),
            Item::new("Backstage passes to a TAFKAL80ETC concert", -24, 0),
            Item::new("Conjured Mana Cake", -26, 0),
        ];

        let mut rose = GildedRose::new(input);
        let days:i32 = 29;

        days_loop(&mut rose,days);
        
        for i in 0..output.len(){
         assert_eq!(
            (output[i].sell_in,output[i].quality),
            (rose.items[i].sell_in,rose.items[i].quality)
         );
        }
   }

}
